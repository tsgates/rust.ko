# rust.ko

a minimal Linux kernel module written in rust.

## Requirements

 - A recent build of Rust (latest nightly)

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

## `make` command-line options

Currently there are a few parameters you can pass to `make` to affect the build:

 * `V=1`: Enable verbose output
   - All commands run by `make` and `cargo` will be printed to the screen
 * `RELEASE=1`: Create a release build
   - `rustc` will compile all code in `--release` mode
   - Debugging information will not be added to the final binary

# Notes

## Cargo.lock

When running `make` for the first time `cargo` will generate a `Cargo.lock` file in the project's
root directory. This file contains information about the exact versions of the dependencies of
your project. Since this is an example project it will not ship any `Cargo.lock` file, but it's
recommended that you commit it to your source tree if you're actually trying to build a project
based on this code.<br />
(Please don't make any pull requests to this code containing a `Cargo.lock` file through.)

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

## Kernel API and ABI stability

Due to the fact that Linux does not provide any ABI stability (not even for two identical copies of
its source code), we must build kernel modules against its A*P*I instead. While this is trivially
done in C, by building the code against the headers of that kernel version, it's not that easy to
do in Rust. Most importantly because Rust cannot read C headers of course, but also because any
`extern "C"` function bindings and, more importantly, any `#[repr(C)]` structure definitions you
define, will link against the kernel's *ABI* not the *API*!

Here is an incomplete list of things that affect the kernel's ABI stability:

 - Using a different version of `gcc` or `clang` to compile the kernel's source code
 - Adding or removing private fields from kernel data structures (may even happen in minor releases!)
 - Changing kernel build options:
    * Some build options cause data structures to contain extra fields
    * Some functions might just disappear
    * Memory alignment or field ordering might change to make data structures more memory efficient

Check out [the kernel documentation](https://git.kernel.org/cgit/linux/kernel/git/torvalds/linux.git/tree/Documentation/stable_api_nonsense.txt)
for more information on kernel API (in)stability.

Currently [`rust-bindgen`](https://github.com/crabtw/rust-bindgen) is invoked at the module's build
time to generate Rust counterparts for all C data structures, enumerations, functions and types
defined by the kernel headers. This addresses most of the issues mentioned above, but is not a
silver bullet: No bindings are generated for C macro definitions and in-line functions. Most
importantly this matters, because, for a C developer it is often irrelevant whether they declare a
bunch of macros for different states of something, or use an `enum` definition instead; after all,
the result is the same, right? While this is certainly true from a C programmers perspective, it is
not the same from the compiler's point of view: C macros are expanded during the preprocessor stage,
while enums are expanded during the main compiling/assembling stage. For `rust-bindgen` (that
operates on the main compiling stage) this means that all macros have disappeared by the time it
gets to look at the source code's [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree). There
is no way to use C macro definitions because of this at the current time.

## Compilation times

The first build will be ratter slow as [`syntex-syntax`](https://github.com/serde-rs/syntex)
(required by [`rust-bindgen`](https://github.com/crabtw/rust-bindgen)) needs to be compiled once to
be able to generate kernel header bindings on the fly. Also, when you target another kernel or
change the list of headers used (with the `KERNEL_INCLUDE` option), you will also experience a
slight delay while the bindings are regenerated.

Please see the [Kernel API and ABI stability](#kernel-api-and-abi-stability) section for details on
why this is necessary for any serious use of the kernel interfaces.


# refs
 - [rustboot](https://github.com/charliesome/rustboot.git)
