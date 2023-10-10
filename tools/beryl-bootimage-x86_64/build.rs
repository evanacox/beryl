//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use bootloader::DiskImageBuilder;
use std::{env, path::PathBuf};

fn main() {
    let kernel_path = env::var("CARGO_BIN_FILE_KERNEL").unwrap();
    let disk_builder = DiskImageBuilder::new(PathBuf::from(kernel_path.clone()));
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let uefi_path = out_dir.join("beryl-x86_64-uefi.img");
    let bios_path = out_dir.join("beryl-x86_64-bios.img");

    disk_builder.create_uefi_image(&uefi_path).unwrap();
    disk_builder.create_bios_image(&bios_path).unwrap();

    println!("cargo:rustc-env=UEFI_IMAGE={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_IMAGE={}", bios_path.display());
}
