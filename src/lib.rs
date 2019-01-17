#![feature(custom_attribute, lang_items, panic_info_message)]
#![no_std]

#[macro_use]
extern crate linux_std as std;

// Defines various language items that need to be around
mod lang;

#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust!++");
}
