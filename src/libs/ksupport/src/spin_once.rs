//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use core::cell::UnsafeCell;
use core::hint::unreachable_unchecked;
use core::intrinsics;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU8, Ordering};
use core::{hint, mem};

/// A thread-safe `OnceCell` that uses an atomic flag
/// to determine initialized/uninitialized.
///
/// If the value isn't initialized and we try to `get` it, we
/// enter a spin loop.
pub struct SpinOnceCell<T> {
    inner: UnsafeCell<MaybeUninit<T>>,
    init: AtomicU8,
}

impl<T> SpinOnceCell<T> {
    const EMPTY: u8 = 0;
    const FULL: u8 = 1;
    const FILLING: u8 = 2;

    /// Creates an uninitialized [`SpinOnceCell`]. The value needs to be
    /// initialized before it can be access.
    pub const fn uninit() -> Self {
        Self {
            inner: UnsafeCell::new(MaybeUninit::uninit()),
            init: AtomicU8::new(Self::EMPTY),
        }
    }

    /// If the value has been initialized, returns a reference to the value.
    ///
    /// Otherwise, spins until it is initialized, then returns a reference
    /// to the value.
    pub fn get(&self) -> &T {
        self.wait_until();

        let inner = self.inner.get();

        unsafe { (*inner).assume_init_ref() }
    }

    /// If the value has been initialized, returns a mutable reference to the value.
    ///
    /// Otherwise, spins until it is initialized, then returns a mutable reference
    /// to the value.
    pub fn get_mut(&mut self) -> &mut T {
        self.wait_until();

        let inner = self.inner.get_mut();

        unsafe { inner.assume_init_mut() }
    }

    /// If the value has been initialized, returns a mutable reference to the value.
    ///
    /// Otherwise, spins until it is initialized, then returns a mutable reference
    /// to the value.
    pub fn get_or_init(&self, init: impl FnOnce() -> T) -> &T {
        let inner = self.inner.get();

        if let Ok(_) = self.init.compare_exchange(
            Self::EMPTY,
            Self::FILLING,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            unsafe {
                let uninit = &mut *self.inner.get();

                uninit.write(init());
            }

            self.init.store(Self::FULL, Ordering::Release);
        }

        unsafe { (*inner).assume_init_ref() }
    }

    /// If the value isn't initialized, initializes it and returns `Ok(())`.
    ///
    /// Otherwise, returns `Err(value)`.
    pub fn set(&self, value: T) -> Result<(), T> {
        match self.init.compare_exchange(
            Self::EMPTY,
            Self::FILLING,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                unsafe {
                    let uninit = &mut *self.inner.get();

                    uninit.write(value);
                }

                self.init.store(Self::FULL, Ordering::Release);

                Ok(())
            }
            Err(_) => Err(value),
        }
    }

    /// If the value is initialized, takes it out and sets `self`
    /// back to the uninitialized state.
    pub fn take(&mut self) -> Option<T> {
        let old = mem::take(self);

        old.into_inner()
    }

    /// Gets the inner value of `self`.
    ///
    /// If it's initialized, returns `Some(value)`. Otherwise, returns `None`.
    pub fn into_inner(self) -> Option<T> {
        unsafe {
            match self.init.load(Ordering::Acquire) {
                Self::EMPTY | Self::FILLING => None,
                Self::FULL => Some(self.inner.into_inner().assume_init()),
                _ => unreachable_unchecked(),
            }
        }
    }

    #[inline(always)]
    fn wait_until(&self) {
        while intrinsics::unlikely(self.init.load(Ordering::Acquire) != Self::FULL) {
            hint::spin_loop();
        }
    }
}

impl<T> Default for SpinOnceCell<T> {
    fn default() -> Self {
        Self::uninit()
    }
}

unsafe impl<T> Sync for SpinOnceCell<T> {}
