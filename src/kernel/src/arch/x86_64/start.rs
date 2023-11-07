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
use bootloader_api::config::{ApiVersion, Mapping};
use bootloader_api::info::MemoryRegionKind;
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
    let mut usable = 0usize;

    for region in info.memory_regions.iter() {
        let (start, end) = (region.start, region.end);

        memory += (end - start) as usize;

        match region.kind {
            MemoryRegionKind::Usable => {
                trace!("found usable page! [{start:0x}, {end:0x}]");

                usable += (end - start) as usize;
            }
            MemoryRegionKind::Bootloader => trace!("found bootloader page! [{start:0x}, {end:0x}]"),
            MemoryRegionKind::UnknownUefi(_) => {
                trace!("found unknown uefi page! [{start:0x}, {end:0x}]")
            }
            MemoryRegionKind::UnknownBios(_) => {
                trace!("found unknown bios page! [{start:0x}, {end:0x}]")
            }
            kind => panic!("unknown type of memory page '{kind:?}'"),
        }
    }

    trace!("kernel address = {:?}", kernel_start as *mut u8);
    trace!("total memory = {memory} (in bytes)");
    trace!("total usable memory = {usable} (in bytes)");
    trace!("total unusable memory = {} (in bytes)", memory - usable);

    crate::kernel_main(SystemInfo { memory })
}
