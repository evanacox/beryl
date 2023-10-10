//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

//! Kernel-level framebuffer driver.
//!
//! This driver is purely for debug purposes, and is meant to be replaced
//! by a user-mode graphics driver. For now, this is for CPU-level interaction
//! with the framebuffer as exposed by the bootloader.
//!
//! This is intended to be linked directly into the kernel.

#![no_std]
#![deny(missing_docs)]
#![deny(missing_abi)]

/// Does a thing
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(1, 1), 2);
    }
}