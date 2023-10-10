//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

//! Models any architecture/target-specific functionality that we
//! want to abstract away in the kernel.
//!
//! To support new targets, ideally the only thing that needs to be
//! produced is a new HAL.

mod spin;

pub use spin::*;
