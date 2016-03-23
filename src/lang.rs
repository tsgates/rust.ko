use core;

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "eh_unwind_resume"]
extern fn eh_unwind_resume() {}

#[lang = "panic_fmt"]
extern fn panic_impl(_: core::fmt::Arguments, _: &'static str, _: u32) -> ! {
	loop{}
}
