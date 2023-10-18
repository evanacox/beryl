//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2023 Evan Cox <evanacox00@gmail.com>. All rights reserved.      //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use std::io::Stdout;
use std::process::{self, Command, Stdio};

fn main() {
    Command::new("qemu-system-x86_64")
        .arg("-s")
        .arg("-S")
        .arg("-drive")
        .arg("format=raw,file=./target/images/beryl-x86_64-uefi.img")
        .arg("-bios")
        .arg(ovmf_prebuilt::ovmf_pure_efi())
        .arg("-serial")
        .arg("stdio")
        .arg("-m")
        .arg("1G")
        .status()
        .unwrap();
}
