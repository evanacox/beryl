//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

#[cfg(target_arch = "x86_64")]
use super::x86_64::interrupts;

#[cfg(target_arch = "x86_64")]
pub type HALInterruptHandler = interrupts::InterruptHandler;

/// A handler for a single interrupt.
///
/// These are what goes into the interrupt handler table
/// for a given architecture, it models the interrupts
/// that the OS actually cares about.
#[derive(Copy, Clone)]
pub struct HALInterruptTable {
    div_by_zero: Option<HALInterruptHandler>,
}
