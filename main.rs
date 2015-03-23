#![feature(no_std)]
#![feature(macro_rules)]
#![no_std]

extern crate core;
extern crate libc;

mod macros;
mod raw;

#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust!");
}
