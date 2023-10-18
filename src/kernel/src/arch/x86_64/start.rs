//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use crate::arch::x86_64::hal::SerialPort;
use crate::arch::SystemInfo;
use crate::drivers::kframebuffer::LinearFramebuffer;
use crate::drivers::{kframebuffer, klog, kserial};
use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use log::{trace, LevelFilter};

static CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();

    config.kernel_stack_size = 8 * 1024 * 1024; // 8 MiB

    config
};

entry_point!(kernel_start, config = &CONFIG);

fn kernel_start(info: &'static mut BootInfo) -> ! {
    kserial::serial_init(|| unsafe { SerialPort::default_com1() });
    klog::logger_init(LevelFilter::Trace);

    trace!("initialized serial");

    kframebuffer::framebuffer_init(|| unsafe {
        LinearFramebuffer::from(info.framebuffer.as_mut().unwrap_unchecked())
    });

    trace!("initialized framebuffer");

    let mut memory = 0usize;

    for region in info.memory_regions.iter() {
        memory += (region.end - region.start) as usize;
    }

    crate::kernel_main(SystemInfo { memory })
}
