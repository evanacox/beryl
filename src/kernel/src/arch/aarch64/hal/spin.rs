//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use core::arch::asm;

/// This is used to halt a thread in kernel mode.
///
/// It relies on the privileged aarch64 instructions
/// `wfi`, and just runs them in an infinite loop.
pub unsafe fn privileged_halt_thread() -> ! {
    loop {
        asm!("wfi");
    }
}
