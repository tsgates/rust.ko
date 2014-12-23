use libc::c_char;

extern {
    pub fn printk(fmt: *const c_char);
}
