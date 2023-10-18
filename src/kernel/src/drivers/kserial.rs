//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

//! Kernel-level serial (UART) driver that integrates with `log`.
//!
//! This is intended to be linked directly into the kernel.

use crate::arch::hal::SerialPort;
use crate::utility::{KSpinFairMutex, KSpinOnceCell};
use core::fmt::Write;
use ksupport::sync::BasicMutex;

/// A UART serial backend that can be used for `log`.
///
/// This is the generic trait, the actual implementation is chosen
/// at build time based on the target architecture.
pub trait SerialBackend: Write {
    /// Initializes the serial port correctly
    fn init(&mut self);

    /// Writes a single byte to the serial backend.
    fn send(&mut self, byte: u8);

    /// Reads a single byte from the serial backend
    fn recv(&mut self) -> u8;
}

static SERIAL_PORT: KSpinOnceCell<KSpinFairMutex<SerialPort>> = KSpinOnceCell::uninit();

/// Initializes the framebuffer with a given function.
///
/// This is meant to be called from the boot code for a given architecture,
/// where the framebuffer is initialized and then is ready to use from then on.
#[inline]
pub fn serial_init(f: impl FnOnce() -> SerialPort) {
    let _ = SERIAL_PORT.set(KSpinFairMutex::new(f()));

    // initialize the port
    SERIAL_PORT.get().lock().init();
}

/// Returns a reference to the lock that guards the serial port.
#[inline]
pub fn serial() -> &'static KSpinFairMutex<SerialPort> {
    SERIAL_PORT.get()
}
