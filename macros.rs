#![macro_escape]

macro_rules! cstr {
    ($arg:expr) => (concat!($arg, "\0"))
}

macro_rules! print {
    ($str:expr) => (unsafe {
        let str = cstr!($str);
        let (ptr, _): (*const libc::c_char, uint) = core::mem::transmute(str);
        raw::printk(ptr);
    })
}

macro_rules! println {
    ($str:expr) => (print!(concat!($str, "\n")))
}
