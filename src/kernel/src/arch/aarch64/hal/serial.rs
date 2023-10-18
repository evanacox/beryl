//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use crate::drivers::kserial::SerialBackend;
use core::fmt;
use core::fmt::Write;

/// An aarch64-specific MMIO serial port.
pub struct SerialPort {
    uart: u64,
}

impl SerialBackend for SerialPort {
    fn init(&mut self) {
        todo!()
    }

    fn send(&mut self, byte: u8) {
        todo!()
    }

    fn recv(&mut self) -> u8 {
        todo!()
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        panic!()
    }
}
