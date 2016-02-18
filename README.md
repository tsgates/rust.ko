# rust.ko

a minimal Linux kernel module written in rust.

## Requirements

 - A recent build of Rust (1.8-nightly)

This code uses [feature flags](http://blog.rust-lang.org/2014/10/30/Stability.html#the-plan), so
you'll need to use a [nightly version of Rust](http://doc.rust-lang.org/book/nightly-rust.html)
to compile it.

## TL;DR

1. Create a file `config.mk` in the root directory of the repository.
2. Set `RUST_ROOT` in this file to the directory containing `bin/rustc`, e.g. `RUST_ROOT := /usr`
3. Compile `rust.ko`:

        $ make

4. Try it out:

        # insmod hello.ko
        # rmmod hello
        $ dmesg | tail -3
          [54024.186997] hello: init
          [54024.187000] hello from rust
          [54024.191963] hello: exit
    
# refs
 - [rustboot](https://github.com/charliesome/rustboot.git)
