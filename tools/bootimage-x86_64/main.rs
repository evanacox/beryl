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
use bpaf::*;
use std::fs;
use std::path::PathBuf;

struct Config {
    kernel: PathBuf,
    bios: PathBuf,
    uefi: PathBuf,
}

fn parse_args() -> Config {
    let kernel = long("kernel")
        .help("path to the kernel '.elf' file to turn into a bootable flat binary")
        .argument::<PathBuf>("PATH");

    let bios = long("bios")
        .help("path to write the BIOS flat binary to")
        .argument::<PathBuf>("PATH");

    let uefi = long("uefi")
        .help("path to write the UEFI flat binary to")
        .argument::<PathBuf>("PATH");

    construct!(Config { kernel, bios, uefi }).to_options().run()
}

fn main() {
    let config = parse_args();

    for output in [&config.bios, &config.uefi] {
        if let Some(parent_of_output) = output.parent() {
            fs::create_dir_all(parent_of_output).expect(&format!(
                "unable to create directory '{}' to write output to",
                parent_of_output.display()
            ));
        }
    }

    let disk_builder = DiskImageBuilder::new(config.kernel.clone());

    disk_builder.create_uefi_image(&config.uefi).unwrap();
    disk_builder.create_bios_image(&config.bios).unwrap();

    println!("uefi image copied to '{}'", config.uefi.display());
    println!("bios image copied to '{}'", config.bios.display());
}
