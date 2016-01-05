#![feature(core_str_ext)]
#![feature(libc)]
#![feature(no_std)]

#![no_std]

extern crate libc;

#[macro_use]
mod macros;
mod raw;

#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust!");
}
