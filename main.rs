#[allow(ctypes)];
#[no_std];
#[no_core];

mod zero;

extern "rust-intrinsic" {
    pub fn transmute<T,U>(val: T) -> U;
}

extern {
    #[fast_ffi]
    pub fn printk(fmt: *u8);
}

unsafe fn print(s: &str) {
    let (ptr, _): (*u8, uint) = transmute(s);
    printk(ptr);
}

#[no_mangle]
pub unsafe fn rust_main() {
    print("hello from rust\n");
}
