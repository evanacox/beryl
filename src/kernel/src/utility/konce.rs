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
use ksupport::SpinOnceCell;

/// Wraps a [`SpinOnceCell`] with correct interrupt/preemption handling.
///
/// Other than that, it works exactly the same.
#[repr(transparent)]
pub struct KSpinOnceCell<T> {
    inner: SpinOnceCell<T>,
}

impl<T> KSpinOnceCell<T> {
    /// Creates an uninitialized [`SpinOnceCell`]. The value needs to be
    /// initialized before it can be access.
    #[inline(always)]
    pub const fn uninit() -> Self {
        Self {
            inner: SpinOnceCell::uninit(),
        }
    }

    /// If the value has been initialized, returns a reference to the value.
    ///
    /// Otherwise, spins until it is initialized, then returns a reference
    /// to the value.
    #[inline(always)]
    pub fn get(&self) -> &T {
        // TODO: interrupts

        self.inner.get()
    }

    /// If the value has been initialized, returns a mutable reference to the value.
    ///
    /// Otherwise, spins until it is initialized, then returns a mutable reference
    /// to the value.
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        // TODO: interrupts

        self.inner.get_mut()
    }

    /// If the value has been initialized, returns a mutable reference to the value.
    ///
    /// Otherwise, spins until it is initialized, then returns a mutable reference
    /// to the value.
    #[inline(always)]
    pub fn get_or_init(&self, init: impl FnOnce() -> T) -> &T {
        // TODO: interrupts

        self.inner.get_or_init(init)
    }

    /// If the value isn't initialized, initializes it and returns `Ok(())`.
    ///
    /// Otherwise, returns `Err(value)`.
    #[inline(always)]
    pub fn set(&self, value: T) -> Result<(), T> {
        // TODO: interrupts

        self.inner.set(value)
    }

    /// If the value is initialized, takes it out and sets `self`
    /// back to the uninitialized state.
    #[inline(always)]
    pub fn take(&mut self) -> Option<T> {
        // TODO: interrupts

        let old = mem::take(&mut self.inner);

        old.into_inner()
    }

    /// Gets the inner value of `self`.
    ///
    /// If it's initialized, returns `Some(value)`. Otherwise, returns `None`.
    #[inline(always)]
    pub fn into_inner(self) -> Option<T> {
        self.inner.into_inner()
    }
}
