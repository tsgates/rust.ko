#[allow(ctypes)];
#[no_std];

extern "rust-intrinsic" {
    pub fn transmute<T,U>(val: T) -> U;
}

extern {
    pub fn printk(fmt: *u8);
}

#[fixed_stack_segment]
unsafe fn print(s: &str) {
    let (ptr, _): (*u8, uint) = transmute(s);
    printk(ptr);
}

#[no_mangle]
pub unsafe fn rust_main() {
    print("hello from rust\n");
}
