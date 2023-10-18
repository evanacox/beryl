//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

//! An x86-64 implementation of the Beryl HAL (hardware abstraction layer).
//!
//! This provides the x86_64-specific implementation of various system
//! functions that the kernel needs to be able to perform.

mod serial;
mod spin;

pub use serial::*;
pub use spin::*;
