//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2023 Evan Cox <evanacox00@gmail.com>. All rights reserved.      //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use bpaf::*;
use std::process;
use std::process::Command;

fn main() {
    let file = positional::<String>("IMAGE").help("the uefi image to boot");
    let mem = short('m')
        .help("the amount of memory to give the vm")
        .argument::<String>("MEMORY")
        .fallback("1G".to_string());

    let (file, mem) = construct!(file, mem).run();

    let status = Command::new("qemu-system-x86_64")
        .arg("-s")
        .arg("-S")
        .arg("-cdrom")
        .arg(&file)
        .arg("-M")
        .arg("q35")
        .arg("-boot")
        .arg("d")
        .arg("-serial")
        .arg("stdio")
        .arg("-m")
        .arg(&mem)
        .status()
        .unwrap();

    process::exit(status.code().unwrap_or(-1));
}
