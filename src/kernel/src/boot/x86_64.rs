//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

//! The boot code for x86-64 targets.
//!  
//! This sets up the HAL and kernel drivers, then jumps to
//! [`kernel_main`](crate::kernel_main).

use crate::boot::BootInfo;
use crate::drivers::kframebuffer::LinearFramebuffer;
use crate::drivers::kserial::SerialPortX86_64;
use crate::drivers::{kframebuffer, klog, kserial};
use bootloader_api::{entry_point, BootInfo as X86_64BootInfo};
use log::{trace, LevelFilter};

entry_point!(kernel_start);

fn kernel_start(info: &'static mut X86_64BootInfo) -> ! {
    kserial::serial_init(|| unsafe { SerialPortX86_64::default_com1() });
    klog::logger_init(LevelFilter::Trace);

    trace!("initialized serial");

    kframebuffer::framebuffer_init(|| unsafe {
        LinearFramebuffer::from(info.framebuffer.as_mut().unwrap_unchecked())
    });

    trace!("initialized framebuffer");

    crate::kernel_main(BootInfo {})
}
