#![allow(improper_ctypes)]
#![no_std]
#![feature(intrinsics)]
#![feature(lang_items)]

extern "rust-intrinsic" {
    pub fn transmute<T,U>(val: T) -> U;
}

extern {
    pub fn printk(fmt: *mut u8);
}

unsafe fn print(s: &str) {
    let (ptr, _): (*mut u8, uint) = transmute(s);
    printk(ptr);
}

#[no_mangle]
#[lang="sized"]
pub unsafe fn rust_main() {
    print("hello from rust\n");
}
