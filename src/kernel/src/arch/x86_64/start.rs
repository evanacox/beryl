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
use crate::arch::{hal, Architecture, SystemInfo};
use crate::drivers::kframebuffer::LinearFramebuffer;
use crate::drivers::{kframebuffer, klog, kserial};
use core::arch::asm;
use limine::{BootInfoRequest, FramebufferRequest, MemmapRequest, StackSizeRequest};
use log::{error, trace, LevelFilter};

const EIGHT_MB_STACK: u64 = 8 * 1024 * 1024;

// set our base revision of the bootloader to revision 1
static LIMINE_BASE_REVISION: [u64; 3] = [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc, 1];

// get the framebuffer from Limine, revision 0 because of limine-rs limitations
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new(0);

// get an 8mb stack
static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new(0).stack_size(EIGHT_MB_STACK);

// get limine info for logging purposes
static BOOT_INFO_REQUEST: BootInfoRequest = BootInfoRequest::new(0);

static MEM_MAP_REQUEST: MemmapRequest = MemmapRequest::new(0);

static mut MANUFACTURER_ID: [u8; 12] = [0; 12];

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

        LinearFramebuffer::from(&mut *buf)
    });

    trace!("initialized framebuffer");
}

fn cpuid() -> bool {
    // get manufacturer ID from `cpuid`
    let mut ebx: u32;
    let mut ecx: u32;
    let mut edx: u32;

    // see https://en.wikipedia.org/wiki/CPUID#Calling_CPUID
    unsafe {
        asm!(
        "push rbx",
        "cpuid",
        "mov [rdi], ebx",
        "mov [rdi + 4], edx",
        "mov [rdi + 8], ecx",
        "pop rbx",
        in("rdi") MANUFACTURER_ID.as_mut_ptr(),
        inout("eax") 0 => _,
        out("ecx") _,
        out("edx") _,
        );
    }

    unsafe {
        asm!("cpuid",
        in("eax") 0x80000001u32,
        out("edx") edx);
    }

    // if the 29th (starting from 0) bit is set, long mode is enabled
    edx & (1 << 29) != 0
}

fn initialize_mem_map() {
    let response = MEM_MAP_REQUEST
        .get_response()
        .get()
        .expect("bootloader did not give a memory map, unable to proceed");

    for i in 0..response.entry_count {
        let entry = unsafe { &**response.entries.as_ptr().offset(i as isize) };

        trace!(
            "found region at i = {i}: (base: {:0x}, len: {:0x}, type: {:?})",
            entry.base,
            entry.len,
            entry.typ
        );
    }
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

    if !cpuid() {
        error!(
            "Beryl only supports x86-64 processors, not x86 processors. \
               The current CPU reported that it doesn't support long mode via `cpuid`"
        );

        unsafe { hal::privileged_halt_thread() }
    }

    initialize_kframebuffer();
    initialize_mem_map();

    crate::kernel_main(SystemInfo {
        cpu: (Architecture::X86_64, unsafe {
            core::str::from_utf8_unchecked(&MANUFACTURER_ID)
        }),
        memory: 0,
    })
}
