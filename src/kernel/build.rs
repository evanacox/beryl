//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use std::env;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    match env::var("TARGET").unwrap().as_str() {
        "x86_64-unknown-none" => {
            let abs = fs::canonicalize("./linker.x86_64-elf.ld") //
                .expect("linker script not found!");

            println!("cargo:rustc-link-arg=-T{}", abs.display());
            println!("cargo:rerun-if-changed=linker.x86_64-elf.ld");
        }
        _ => panic!("unknown target!"),
    }
}
