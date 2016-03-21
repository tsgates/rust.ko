

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
#[no_mangle]
#[allow(non_snake_case)]
pub fn _Unwind_Resume(_ix_obj: *mut ()) {}
