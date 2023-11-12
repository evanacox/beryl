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
use core::arch::asm;
use limine::{BootInfoRequest, Framebuffer, FramebufferRequest, StackSizeRequest};
use log::{trace, LevelFilter};

const EIGHT_MB_STACK: u64 = 8 * 1024 * 1024;

// set our base revision of the bootloader to revision 1
static LIMINE_BASE_REVISION: [u64; 3] = [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc, 1];

// get the framebuffer from Limine, revision 0 because of limine-rs limitations
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new(0);

// get an 8mb stack
static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new(0).stack_size(EIGHT_MB_STACK);

// get limine info for logging purposes
static BOOT_INFO_REQUEST: BootInfoRequest = BootInfoRequest::new(0);

fn initialize_klog() {
    kserial::serial_init(|| unsafe { SerialPort::default_com1() });
    klog::logger_init(LevelFilter::Trace);

    trace!("initialized serial");
}

fn initialize_kframebuffer() {
    let mut response = FRAMEBUFFER_REQUEST.get_response();
    let framebuffer = response
        .get_mut()
        .expect("should get a response from limine");

    trace!(
        "got framebuffers! count = {}",
        framebuffer.framebuffer_count
    );

    assert!(
        framebuffer.framebuffer_count >= 1,
        "must have at least one framebuffer"
    );

    kframebuffer::framebuffer_init(|| unsafe {
        let buf = framebuffer.framebuffers()[0].as_ptr();

        trace!("first framebuffer = {:?}", &*buf);

        LinearFramebuffer::from(&mut *buf)
    });

    trace!("initialized framebuffer");
}

#[no_mangle]
extern "C" fn _start() -> ! {
    if LIMINE_BASE_REVISION[2] != 0 {
        // if the bootloader didn't understand our revision, we're too far gone
        unsafe {
            asm!("cli");

            loop {
                asm!("hlt");
            }
        }
    }

    initialize_klog();
    initialize_kframebuffer();

    let mut memory = 0usize;
    let mut usable = 0usize;

    /*

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
    }*/

    trace!("kernel address = {:?}", _start as *mut u8);
    trace!("total memory = {memory} (in bytes)");
    trace!("total usable memory = {usable} (in bytes)");
    trace!("total unusable memory = {} (in bytes)", memory - usable);

    crate::kernel_main(SystemInfo { memory })
}
