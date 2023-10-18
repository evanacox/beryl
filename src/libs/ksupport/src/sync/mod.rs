//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

//! Defines basic synchronization primitives that can be used in kernel-level
//! code.
//!
//! These may not necessarily be safe to use *directly* (e.g. mutexes are
//! wrapped to ensure safe interrupt handling inside the kernel), but they
//! are always at least able to be used in both kernel and user mode.

mod basic_mutex;
mod spin_mutex;

pub use basic_mutex::*;
pub use spin_mutex::{SpinFairMutex, SpinMutex};
