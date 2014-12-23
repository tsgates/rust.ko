#![no_std]

extern crate core;
extern crate libc;

use core::mem::transmute;
use libc::c_char;

extern {
    pub fn printk(fmt: *const c_char);
}

unsafe fn print(s: &str) {
    let (ptr, _): (*const c_char, uint) = transmute(s);
    printk(ptr);
}

#[no_mangle]
pub unsafe fn rust_main() {
    print("hello from rust\n");
}
