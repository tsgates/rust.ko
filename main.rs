#![allow(improper_ctypes)]
#![no_std]
#![feature(intrinsics)]

extern crate core;

use core::mem::transmute;

extern {
    pub fn printk(fmt: *mut u8);
}

unsafe fn print(s: &str) {
    let (ptr, _): (*mut u8, uint) = transmute(s);
    printk(ptr);
}

#[no_mangle]
pub unsafe fn rust_main() {
    print("hello from rust\n");
}
