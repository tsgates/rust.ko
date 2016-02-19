use types;

extern {
    pub fn printk(fmt: *const types::c_char);
}
