//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2023 Evan Cox <evanacox00@gmail.com>. All rights reserved.      //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

//! The Beryl µkernel.
//!
//! There is no public (Rust-level) API exposed from the kernel, everything
//! is exposed through the syscall interface and the associated SDK.

#![no_std]
#![no_main]
#![deny(missing_docs)]
#![deny(missing_abi)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::mod_module_files, clippy::pub_use)]
#![feature(abi_x86_interrupt)]

mod boot;
mod drivers;
mod hal;
mod utility;

use crate::boot::BootInfo;
use crate::drivers::kframebuffer;
use core::panic::PanicInfo;
use core::{ptr, slice};
use ksupport::sync::BasicMutex;
use log::{error, trace};

/// The true platform-independent entry point for the kernel.
///
/// Boot code (in the `boot/` subdirectory) sets up the kernel drivers and any necessary state,
/// then they call this function with information they collect in their platform-dependent
/// way.
///
/// At this point, the stack is expected to be set up, drivers initialized, anything else
/// that is "reasonable" to use is ready (except floating-point).
pub fn kernel_main(_: BootInfo) -> ! {
    trace!("entered kernel::kernel_main");

    let mut buf = kframebuffer::framebuffer();

    {
        let mut raw = buf.lock();

        for at in 0..4096000usize {
            let byte = match at & 3 {
                0 => 255,
                1 => 81,
                2 => 70,
                _ => 255,
            };

            raw.raw_set(at, byte);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{info}");

    unsafe {
        hal::privileged_halt_thread();
    }
}
