#![feature(core_str_ext)]
#![feature(custom_attribute, lang_items)]
#![no_std]

#[macro_use]
mod macros;

mod raw;

// Defines various symbols that need to be around.
mod lang;

pub mod types;

#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust!++");
}
