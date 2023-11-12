# Debugging Beryl

## x86-64 w/ `gdb`

Beryl uses Limine on x86-64, and as such is pretty trivial to debug.

Use the `qemu-x86_64-debuggable` target, and then run a remote `gdb` session (or
a remote debug using e.g. CLion) targeting `localhost:1234`.

In one terminal (or in the background):

```shell 
$ cargo build --kernel --target x86_64-unknown-none
$ cargo run --bin bootimage-x86_64 -- --kernel ./target/x86_64-unknown-none/debug/kernel \
                                      --iso ./target/images/beryl-x86_64-debug.iso
$ cargo run --bin qemu-x86_64-debuggable -- ./target/images/beryl-x86_64-debug.iso
```

In another shell:
```shell
$ gdb
gdb) file ./target/x86_64-unknown-none/debug/kernel
gdb) target remote localhost:1234
gdb) # whatever setup you want, breakpoints, whatever
gdb) continue
```