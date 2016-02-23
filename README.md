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

# Notes

## Cargo.lock

When running `make` for the first time `cargo` will generate a `Cargo.lock` file in the project's
root directory. This file contains information about the exact versions of the dependencies of
your project. Since this is an example project it will not ship any `Cargo.lock` file, but it's
recommended that you commit it to your source tree if you're actually trying to build a project
based on this code.<br />
(Please don't make any pull requests to this code containing a `Cargo.lock` file through.)

## File size

If you look at the file size of your `hello.ko` file, you'll notice that it's pretty big (~100kB)
for a dummy module that simply prints 3 lines to the kernel ring buffer (*dmesg*). This is mostly
because the default build settings produce a debugging build that contains a lot of redundant
debugging information. If you're concerned about the file size, try doing a release build, using
`make RELEASE=1`, instead and you'll find the file size to be a lot more reasonable (~5kB).

## Build targets

Since Linux code can be compiled for a lot of different architectures, we have to be able to
generate CPU code that is in line with what the kernel expects. This means specifically:

 * No floating-point operations in kernel mode **at all**
 * No CPU instructions that write to floating-point registers (*SSE*, *SMID*, … on x86 for instance)
 * No usage of the [red zone](https://en.wikipedia.org/wiki/Red_zone_%28computing%29)
 * No target operating system (after all: When you are in kernel mode, *you* are the operating system)

Currently this source code only ships with a target specification file for the
[x64_64](x86_64-unknown-none-gnu.json) architecture. If you get an error similar to the following,
you'll have the honor of creating and submitting one for favourite architecture: :wink:

	cd "…/rust.ko" && /usr/local/bin/cargo rustc --target="armhf-unknown-none-gnu" --  --emit obj -o "…/rust.ko/build/hello-rust.o"
	failed to run `rustc` to learn about target-specific information

Some ideas on how to do this may be found
[here](http://www.randomhacks.net/2015/11/11/bare-metal-rust-custom-target-kernel-space/) under the
*Creating a target file* section.

# refs
 - [rustboot](https://github.com/charliesome/rustboot.git)
