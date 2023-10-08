//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2023 Evan Cox <evanacox00@gmail.com>. All rights reserved.      //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(_info: &'static mut BootInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}