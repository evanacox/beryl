# beryl
BerylOS: A toy operating system

## Building

To build the operating system, you need to first build the `kernel` package
for the target operating system you want, and then you need to use the 
`bootimage-<arch>` target to turn the build output from `kernel` (an ELF file)
into a flat binary (a `.img` file).

> Note: If you try to `cargo run` some of the tools and they spew out errors
> from the standard library, try adding `--target <your target triple>` to force
> `cargo` to build the executable for your host system.
> 
> This shouldn't be necessary, but if you have a `.cargo/config.toml` that 
> has a target configured for IDE reasons it can be.

### Ex: Building for x86-64 in release mode

Requirements for `bootimage-x86_64`:

1. `xorriso`
2. `git`

Note that both must be in PATH.

```sh
$ cargo build --package kernel --target x86_64-unknown-none --release
$ cargo run --bin bootimage-x86_64 -- \
            --kernel ./target/x86_64-unknown-none/release/kernel \
            --iso ./target/images/beryl-x86_64.iso
```

## Running via QEMU

The project provides some pre-made scripts for launching QEMU. Assuming you've already
built your images with the commands above, you can launch QEMU like this:

```shell
$ cargo run --bin qemu-x86_64-uefi -- ./target/images/beryl-x86_64.iso -m 1G
```

## Project Structure

`sdk/` holds the source code for public-facing libraries, i.e. interfaces that
the operating system itself exposes.

`src/` holds the source code for all standalone/internal code, i.e. code that
turns into an executable (user-mode drivers, the kernel itself, user-mode programs)
or code that is linked directly into the kernel (kernel-mode drivers mostly)

## License

Licensed under the BSD-3 license, see [LICENSE](./LICENSE)
