# rust.ko

a minimal Linux kernel module written in rust.

# TL;DR
To compile you'll need a recent build of Rust (0.13-dev).

Create a file `config.mk` in the root directory of the repository.

Set `RUST_ROOT` in this file to the directory containing `bin/rustc`, e.g.
`RUST_ROOT := /usr`. Then you can compile rust.ko:

    $ make
    # insmod hello.ko
    # rmmod hello
    $ dmesg | tail -3
      [54024.186997] hello: init
      [54024.187000] hello from rust
      [54024.191963] hello: exit

    $ cat main.rs
    
# refs
 - [rustboot](https://github.com/charliesome/rustboot.git)
