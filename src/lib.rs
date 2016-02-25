#![feature(core_str_ext)]
#![feature(custom_attribute)]
#![no_std]


#[macro_use]
mod macros;

mod raw;

pub mod types;

#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust!++");
}
