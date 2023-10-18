//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use core::mem;
use ksupport::sync::{BasicMutex, MutexGuard, SpinFairMutex, SpinMutex};

/// Wraps a [`SpinMutex<T>`] and adds interrupt
/// handling to make sure that interrupts don't screw up locking/unlocking.
///
/// Other than that, everything true about `SpinMutex<T>` is true here.
#[repr(transparent)]
pub struct KSpinMutex<T> {
    inner: SpinMutex<T>,
}

/// Wraps a [`SpinFairMutex<T>`] and adds interrupt
/// handling to make sure that interrupts don't screw up locking/unlocking.
///
/// Other than that, everything true about `SpinFairMutex<T>` is true here.
#[repr(transparent)]
pub struct KSpinFairMutex<T> {
    inner: SpinFairMutex<T>,
}

macro_rules! kmutex_wrapper {
    ($name:ident, $inner:ident) => {
        impl<T> $name<T> {
            #[doc = concat!("Wraps [`", stringify!($name), "::new`]. Nothing changes here, the lock")]
            #[doc = "is created in an unlocked state, and the value given is the initial"]
            #[doc = "value for the held object."]
            pub const fn new(value: T) -> Self {
                Self {
                    inner: $inner::new(value),
                }
            }
        }

        impl<T> BasicMutex<T> for $name<T> {
            #[inline(always)]
            fn lock(&self) -> MutexGuard<'_, Self, T> {
                // TODO: disable interrupts, disable preemption, etc

                let inner = self.inner.lock();

                // once we've locked the inner lock, drop the guard
                // so that we don't unlock `inner` when we make a new guard
                mem::forget(inner);

                unsafe { MutexGuard::new_from_unlocked(self) }
            }

            #[inline(always)]
            fn try_lock(&self) -> Option<MutexGuard<'_, Self, T>> {
                self.inner.try_lock().map(|guard| {
                    // TODO: disable interrupts, disable preemption, etc

                    mem::forget(guard);

                    unsafe { MutexGuard::new_from_unlocked(self) }
                })
            }

            #[inline(always)]
            fn unlock(&self, guard: MutexGuard<'_, Self, T>) {
                mem::forget(guard);

                self.inner
                    .unlock(unsafe { MutexGuard::new_from_unlocked(&self.inner) })
            }

            #[inline(always)]
            unsafe fn unlock_unchecked(&self) {
                self.inner.unlock_unchecked();

                // TODO: re-enable interrupts, re-enable preemption, etc
            }

            #[inline(always)]
            unsafe fn data_unguarded(&self) -> &T {
                self.inner.data_unguarded()
            }

            #[inline(always)]
            unsafe fn data_mut_unguarded(&self) -> &mut T {
                self.inner.data_mut_unguarded()
            }
        }

        impl<T: Default> Default for $name<T> {
            fn default() -> Self {
                Self {
                    inner: $inner::default(),
                }
            }
        }
    };
}

kmutex_wrapper!(KSpinMutex, SpinMutex);

kmutex_wrapper!(KSpinFairMutex, SpinFairMutex);
