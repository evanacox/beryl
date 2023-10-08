# beryl
BerylOS: A toy operating system

## Building

```sh
$ cargo build
$ cargo run --bin beryl-post-build
```

After this is done, `target/<profile>/uefi.img` and `target/<profile>/bios.img` will
both exist and be ready for usage.

## License

Licensed under the BSD-3 license, see [LICENSE](./LICENSE)
