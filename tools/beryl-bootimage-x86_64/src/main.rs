//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use std::{env, fs};

fn main() {
    let output = env::current_dir().unwrap().join("target/images/");

    fs::create_dir_all(&output).expect("unable to create directory ./target/images/");

    let elf_target = output.join("beryl-x86_64.elf");
    let uefi_target = output.join("beryl-x86_64-uefi.img");
    let bios_target = output.join("beryl-x86_64-bios.img");

    fs::copy(env!("ELF_IMAGE"), &elf_target).unwrap();
    fs::copy(env!("UEFI_IMAGE"), &uefi_target).unwrap();
    fs::copy(env!("BIOS_IMAGE"), &bios_target).unwrap();

    println!("beryl: elf binary copied to {}", elf_target.display());
    println!("beryl: uefi image copied to {}", uefi_target.display());
    println!("beryl: bios image copied to {}", bios_target.display());
}
