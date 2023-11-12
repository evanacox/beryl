//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use bpaf::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

struct Config {
    kernel: PathBuf,
    iso: PathBuf,
    force: bool,
}

fn parse_args() -> Config {
    let kernel = long("kernel")
        .help("path to the kernel '.elf' file to turn into a bootable flat binary")
        .argument::<PathBuf>("PATH");

    let iso = long("iso")
        .help("path to write the hybrid ISO to")
        .argument::<PathBuf>("PATH");

    let force = long("force")
        .short('f')
        .help("allows `--iso` to be an existing file, forcing it to be overwritten")
        .flag(true, false);

    construct!(Config { kernel, iso, force }).to_options().run()
}

fn main() {
    let config = parse_args();
    let limine = Path::new("./target/limine/");

    if !config.kernel.exists() {
        panic!("kernel input '{}' does not exist!", config.kernel.display())
    }

    if config.iso.exists() && !config.force {
        panic!(
            "iso input '{}' already exists, re-run with `--force` (or `-f`) to allow overwriting!",
            config.iso.display()
        )
    }

    if !limine.exists() {
        println!("`git clone`-ing a copy of `limine`...");

        // clone binary release of limine
        let _ = Command::new("git")
            .arg("clone")
            .arg("https://github.com/limine-bootloader/limine.git")
            .arg("--branch=v5.x-branch-binary")
            .arg("--depth=1")
            .arg(&limine)
            .output()
            .expect("failed to execute process");

        println!("building the `limine` tool...");

        // build the binary release into `OUT_DIR/limine`
        let _ = Command::new("make")
            .arg("-C")
            .arg(&limine)
            .output()
            .expect("failed to execute process");
    }

    if let Some(parent_of_output) = config.iso.parent() {
        fs::create_dir_all(parent_of_output).expect(&format!(
            "unable to create directory '{}' to write output to",
            parent_of_output.display()
        ));
    }

    // xorriso -as mkisofs -b limine-bios-cd.bin
    //         -no-emul-boot
    //         -boot-load-size 4
    //         -boot-info-table
    //         --efi-boot limine-uefi-cd.bin
    //         -efi-boot-part
    //         --efi-boot-image
    //         --protective-msdos-label iso_root
    //         -o beryl-x86_64-hybrid.iso
    // ./limine/limine bios-install $(IMAGE_NAME).iso
    // rm -rf iso_root
    let iso_root = Path::new("./target/__iso_root/");
    let output = Path::new("./target/images/beryl-x86_64-hybrid.iso");

    println!("building hybrid iso...");
    fs::create_dir_all(iso_root).unwrap();
    copy_files_into_root(iso_root, limine, &config);
    copy_bootloader_files(iso_root, limine);

    build_hybrid_iso(iso_root, output);

    fs::remove_dir_all(iso_root).unwrap();

    println!("hybrid iso copied to '{}'", output.display());
}

fn copy_files_into_root(iso_root: &Path, limine: &Path, config: &Config) {
    // cp (files) iso_root/
    for file in [
        "limine-bios.sys",
        "limine-bios-cd.bin",
        "limine-uefi-cd.bin",
    ] {
        fs::copy(limine.join(file), iso_root.join(file)).unwrap();
    }

    fs::copy(&config.kernel, iso_root.join("beryl.elf")).unwrap();
    fs::copy(
        Path::new("./src/kernel/limine.cfg"),
        iso_root.join("limine.cfg"),
    )
    .unwrap();
}

fn copy_bootloader_files(iso_root: &Path, limine: &Path) {
    let boot = iso_root.join("EFI/BOOT/");

    fs::create_dir_all(&boot).unwrap();

    // cp (files) iso_root/EFI/BOOT
    for file in ["BOOTX64.EFI", "BOOTIA32.EFI"] {
        let mut efi = File::open(limine.join(file)).unwrap();
        let mut out = File::create(boot.join(file)).unwrap();

        std::io::copy(&mut efi, &mut out).unwrap();
    }
}

fn build_hybrid_iso(iso_root: &Path, output: &Path) {
    // xorriso -as mkisofs -b limine-bios-cd.bin
    //         -no-emul-boot
    //         -boot-load-size 4
    //         -boot-info-table
    //         --efi-boot limine-uefi-cd.bin
    //         -efi-boot-part
    //         --efi-boot-image
    //         --protective-msdos-label iso_root
    //         -o beryl-x86_64-hybrid.iso
    Command::new("xorriso")
        .arg("-as")
        .arg("mkisofs")
        .arg("-b")
        .arg("limine-bios-cd.bin")
        .arg("-no-emul-boot")
        .arg("-boot-load-size")
        .arg("4")
        .arg("-boot-info-table")
        .arg("--efi-boot")
        .arg("limine-uefi-cd.bin")
        .arg("-efi-boot-part")
        .arg("--efi-boot-image")
        .arg("--protective-msdos-label")
        .arg(iso_root)
        .arg("-o")
        .arg(output)
        .output()
        .unwrap();
}
