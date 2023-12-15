//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

//! Architecture-specific code, this is where the OS actually boots to
//! before being transferred into platform-independent code in `src/main.rs`.
//!
//! This code is responsible for things like setting up interrupt tables,
//! setting up paging, initializing drivers, etc.
//!
//! This module also provides `hal`

/// The architecture that the kernel is running on
#[derive(Copy, Clone, Debug)]
pub enum Architecture {
    Aarch64,
    X86_64,
}

/// Generic system information that the kernel gets
/// from the arch-specific boot code.
#[derive(Copy, Clone, Debug)]
pub struct SystemInfo {
    /// The CPU architecture that the kernel is running on
    pub cpu: (Architecture, &'static str),
    /// The amount of memory (in bytes) that the host system has
    /// available to it **in total**. This includes memory that
    /// the kernel is currently occupying.
    pub memory: usize,
}

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::hal;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::hal;
