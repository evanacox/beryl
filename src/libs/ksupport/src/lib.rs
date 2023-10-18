//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

//! Kernel-level support library containing utilities needed
//! throughout kernel-level code.
//!
//! This is the "catch-all" crate for that type of code.

#![no_std]
#![feature(core_intrinsics)]
#![deny(missing_docs)]
#![deny(missing_abi)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::mod_module_files, clippy::pub_use)]

mod spin_once;
pub mod sync;
mod xorshift128p;
mod xoshiro256ss;

pub use spin_once::SpinOnceCell;
pub use xorshift128p::Xorshift128Plus;
pub use xoshiro256ss::Xoshiro256;
