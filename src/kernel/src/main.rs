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

mod hal;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

struct Xorshiro256SSState {
    state: [u64; 4],
}

impl Xorshiro256SSState {
    fn next(&mut self) -> u64 {
        let result = self.state[1].rotate_left(7) * 9;
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        result
    }
}

fn kernel_main(info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = info.framebuffer.as_mut() {
        let mut state = Xorshiro256SSState {
            state: [
                0x243F6A8885A308D3,
                0x13198A2E03707344,
                0xA4093822299F31D0,
                0x082EFA98EC4E6C89,
            ],
        };

        for bytes in framebuffer.buffer_mut().chunks_mut(8) {
            let next = state.next();
            let next_bytes = next.to_le_bytes();

            for i in 0..8 {
                bytes[i] = next_bytes[i];
            }
        }
    }

    unsafe {
        hal::privileged_halt_thread();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        hal::privileged_halt_thread();
    }
}
