//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use crate::sync::basic_mutex::{BasicMutex, MutexGuard};
use core::cell::UnsafeCell;
use core::hint;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// A basic spinlock-based mutex.
///
/// The algorithm is not fair at all, it's purely a matter of
/// which threads get lucky enough to grab the lock once it
/// becomes unlocked.
///
/// This is not interrupt-safe, kernel use must wrap interrupt
/// handling code around this to use it safely.
pub struct SpinMutex<T> {
    data: UnsafeCell<T>,
    locked: AtomicBool,
}

impl<T> SpinMutex<T> {
    /// Creates a new mutex instance with a given initial value
    /// for the held object.
    ///
    /// The lock starts in the "unlocked" state.
    pub const fn new(value: T) -> Self {
        Self {
            data: UnsafeCell::new(value),
            locked: AtomicBool::new(false),
        }
    }
}

impl<T> BasicMutex<T> for SpinMutex<T> {
    fn lock(&self) -> MutexGuard<'_, Self, T> {
        // this is a TTAS loop. If the lock is already unlocked, we
        // just take it immediately, otherwise we just keep trying to load
        // until it becomes unlocked (then we attempt to lock it again).
        //
        // this prevents us from screwing with the CPU's cache coherency model,
        // as multiple threads won't constantly be fighting over a single
        // cache line trying to store `true` into it.
        loop {
            if !self.locked.swap(true, Ordering::Acquire) {
                return unsafe { MutexGuard::new_from_unlocked(self) };
            }

            while self.locked.load(Ordering::Relaxed) {
                hint::spin_loop();
            }
        }
    }

    fn try_lock(&self) -> Option<MutexGuard<'_, Self, T>> {
        // pre-emptively load, helps in the case where `try_lock` is in a loop
        if !self.locked.load(Ordering::Relaxed) && !self.locked.swap(true, Ordering::Acquire) {
            Some(unsafe { MutexGuard::new_from_unlocked(self) })
        } else {
            None
        }
    }

    unsafe fn unlock_unchecked(&self) {
        // unlock the atomic variable
        self.locked.store(false, Ordering::Release);
    }

    unsafe fn data_unguarded(&self) -> &T {
        &*self.data.get()
    }

    unsafe fn data_mut_unguarded(&self) -> &mut T {
        &mut *self.data.get()
    }
}

unsafe impl<T> Send for SpinMutex<T> {}

unsafe impl<T> Sync for SpinMutex<T> {}

impl<T: Default> Default for SpinMutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// A basic spinlock-based mutex that is also "fair", i.e.
/// the thread that started waiting first gets the lock first.
///
/// The algorithm is the standard "ticket lock" model, where waiters are
/// given a "ticket" and the mutex keeps track of which ticket is currently
/// able to access the data. Once a thread unlocks, the next ticket is
/// given a chance to lock the mutex.
///
/// This is not interrupt-safe, kernel use must wrap interrupt
/// handling code around this to use it safely.
pub struct SpinFairMutex<T> {
    data: UnsafeCell<T>,
    count: AtomicUsize,
    current: AtomicUsize,
}

impl<T> SpinFairMutex<T> {
    /// Creates a new mutex instance with a given initial value
    /// for the held object.
    ///
    /// The lock starts in the "unlocked" state.
    pub const fn new(value: T) -> Self {
        Self {
            data: UnsafeCell::new(value),
            count: AtomicUsize::new(0),
            current: AtomicUsize::new(0),
        }
    }
}
impl<T> BasicMutex<T> for SpinFairMutex<T> {
    fn lock(&self) -> MutexGuard<'_, Self, T> {
        // `fetch_add` wraps on overflow, so unless we have 2^64 different waiters
        // at the same time we won't have any issues (translation: we won't have issues)
        let ticket = self.count.fetch_add(1, Ordering::Relaxed);

        // similar TTAS loop to SpinMutex, just with tickets instead
        loop {
            if self.current.load(Ordering::Acquire) == ticket {
                return unsafe { MutexGuard::new_from_unlocked(self) };
            }

            while self.current.load(Ordering::Relaxed) != ticket {
                hint::spin_loop();
            }
        }
    }

    fn try_lock(&self) -> Option<MutexGuard<'_, Self, T>> {
        // effectively, this says "try and see if the lock is able to be locked right now,
        // if it is, acquire the lock, otherwise bail out"
        let ticket = self
            .count
            // we `acquire` on `current` and not `count`, see `unlock_unchecked`
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |ticket| {
                if self.current.load(Ordering::Acquire) == ticket {
                    Some(ticket + 1)
                } else {
                    None
                }
            });

        // if we got the lock, return Some(guard). otherwise, None.
        ticket
            .ok()
            .map(|_| unsafe { MutexGuard::new_from_unlocked(self) })
    }

    unsafe fn unlock_unchecked(&self) {
        self.current.fetch_add(1, Ordering::Release);
    }

    unsafe fn data_unguarded(&self) -> &T {
        &*self.data.get()
    }

    unsafe fn data_mut_unguarded(&self) -> &mut T {
        &mut *self.data.get()
    }
}

unsafe impl<T> Send for SpinFairMutex<T> {}

unsafe impl<T> Sync for SpinFairMutex<T> {}

impl<T: Default> Default for SpinFairMutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
