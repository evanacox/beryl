//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use std::process::{self, Command};

fn main() {
    let mut status = Command::new("qemu-system-x86_64")
        .arg("-drive")
        .arg("format=raw,file=./target/images/beryl-x86_64-bios.img")
        .arg("-serial")
        .arg("stdio")
        .status()
        .unwrap();

    process::exit(status.code().unwrap_or(-1));
}
