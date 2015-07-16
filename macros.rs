macro_rules! cstr {
    ($arg:expr) => (concat!($arg, "\0"))
}

macro_rules! print {
    ($str:expr) => ({
        use core::str::StrExt;
        let str = cstr!($str);
        let ptr = str.as_ptr() as *const libc::c_char;
        unsafe { raw::printk(ptr); }
    })
}

macro_rules! println {
    ($str:expr) => (print!(concat!($str, "\n")))
}
