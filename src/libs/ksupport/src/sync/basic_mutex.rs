//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

/// An RAII lock guard that gives access to the
/// data that a mutex is protecting.
pub struct MutexGuard<'mutex, Underlying, T>
where
    Underlying: BasicMutex<T>,
{
    mutex: &'mutex Underlying,
    _unused: PhantomData<T>,
}

impl<'mutex, Underlying, T> MutexGuard<'mutex, Underlying, T>
where
    Underlying: BasicMutex<T>,
{
    /// Creates a new [`MutexGuard`] from an unlocked mutex.
    ///
    /// # Safety
    /// `mutex` **must** already be locked, or the behavior is undefined.
    #[inline(always)]
    pub unsafe fn new_from_unlocked(mutex: &'mutex Underlying) -> Self {
        Self {
            mutex,
            _unused: PhantomData::default(),
        }
    }
}

impl<'mutex, Underlying, T> Drop for MutexGuard<'mutex, Underlying, T>
where
    Underlying: BasicMutex<T>,
{
    fn drop(&mut self) {
        unsafe {
            self.mutex.unlock_unchecked();
        }
    }
}

impl<'mutex, Underlying, T> Deref for MutexGuard<'mutex, Underlying, T>
where
    Underlying: BasicMutex<T>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.mutex.data_unguarded() }
    }
}

impl<'mutex, Underlying, T> DerefMut for MutexGuard<'mutex, Underlying, T>
where
    Underlying: BasicMutex<T>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.mutex.data_mut_unguarded() }
    }
}

/// A basic kernel mutex.
///
/// These function like the standard [`Mutex`](std::sync::Mutex), where they contain
/// a value in an [`UnsafeCell<T>`](std::cell::UnsafeCell) and ensure only one thread
/// can access that value at a time.
pub trait BasicMutex<T>: Send + Sync + Sized {
    /// Locks the mutex and returns a guard that provides
    /// access to the underlying data.
    ///
    /// If the mutex is already locked, waits in a mutex-specific
    /// way until the mutex is unlocked.
    ///
    /// "Fairness" is lock-dependent.
    fn lock(&self) -> MutexGuard<'_, Self, T>;

    /// Attempts to lock the mutex. If it's unlocked, locks it
    /// and returns `Some(guard)`. If it's locked by another thread,
    /// returns `None`.
    fn try_lock(&self) -> Option<MutexGuard<'_, Self, T>>;

    /// Unlocks the mutex in a way that's more readable than `drop(guard)`.
    ///
    /// The lock becomes available to any other threads waiting on it.
    #[inline(always)]
    fn unlock(&self, guard: MutexGuard<'_, Self, T>) {
        drop(guard);
    }

    /// Unlocks the mutex, invalidating the guard previously given
    /// from [`Self::lock`].
    ///
    /// This should not need to be called normally, it is automatically
    /// called whenever the `MutexGuard` is dropped.
    ///
    /// # Safety
    /// The lock must currently be held to be unlocked, and the [`MutexGuard`]
    /// that was returned by [`Self::lock`] must not be dropped AND must not
    /// be accessed again.
    ///
    /// You almost certainly do not need to use this, use [`Self::unlock`]
    /// instead.
    unsafe fn unlock_unchecked(&self);

    /// Provides immutable accesses to the underlying data from
    /// the [`UnsafeCell<T>`](std::cell::UnsafeCell).
    ///
    /// # Safety
    /// `self` must be locked, or it must be impossible for more than one thread
    /// to call this at the same time.
    ///
    /// This condition is unchecked.
    unsafe fn data_unguarded(&self) -> &T;

    /// Provides mutable accesses to the underlying data from
    /// the [`UnsafeCell<T>`](std::cell::UnsafeCell).
    ///
    /// # Safety
    /// `self` must be locked, or it must be impossible for more than one thread
    /// to call this at the same time.
    ///
    /// This condition is unchecked.
    unsafe fn data_mut_unguarded(&self) -> &mut T;
}
