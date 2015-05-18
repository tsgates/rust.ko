#![feature(core)]
#![feature(libc)]
#![feature(no_std)]

#![no_std]

extern crate core;
extern crate libc;

#[macro_use]
mod macros;
mod raw;

#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust!");
}
