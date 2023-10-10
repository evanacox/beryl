# beryl
BerylOS: A toy operating system

## Building

`beryl-bootimage-<arch>` packages are provided that build and package the operating
system for different targets. Choose your target, and figure out what your host 
architecture is (since you need to specify it). Then, run the `bootimage` target.

> Note: you must be in the root directory of the project when running these,
> or the executable will be put in the wrong directory.
 
Ex: building for x86-64 on x86-64 Linux host `x86_64-unknown-linux-gnu`:

```sh
beryl $ cargo run --package beryl-bootimage-x86_64 --target x86_64-unknown-linux-gnu
```

Ex: building for x86-64 on `aarch64-apple-darwin`:

```sh
beryl $ cargo run --package beryl-bootimage-x86_64 --target x86_64-unknown-linux-gnu
```

The location of the `.img` file will be printed by the executable after it is moved 
into the final location.

## Project Structure

`sdk/` holds the source code for public-facing libraries, i.e. interfaces that
the operating system itself exposes.

`src/` holds the source code for all standalone/internal code, i.e. code that
turns into an executable (user-mode drivers, the kernel itself, user-mode programs)
or code that is linked directly into the kernel (kernel-mode drivers mostly)

## License

Licensed under the BSD-3 license, see [LICENSE](./LICENSE)
