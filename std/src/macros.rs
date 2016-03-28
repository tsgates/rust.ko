#[macro_export]
macro_rules! c_str {
	($arg:expr) => (concat!($arg, '\x00'))
}

#[macro_export]
macro_rules! print {
	// Static (zero-allocation) implementation that uses compile-time `concat!()` only
	($fmt:expr) => ({
		let msg = c_str!($fmt);
		let ptr = msg.as_ptr() as *const ::std::os::raw::c_char; 
		unsafe {
			::std::os::kernel::printk(ptr);
		}
	});
	
	// Dynamic implementation that processes format arguments
	($fmt:expr, $($arg:tt)*) => ({
		use ::core::fmt::Write;
		use ::std::io::KernelDebugWriter;
		
		let mut writer = KernelDebugWriter {};
		writer.write_fmt(format_args!($fmt, $($arg)*)).unwrap();
	});
}

#[macro_export] 
macro_rules! println {
	($fmt:expr)              => (print!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)+) => (print!(concat!($fmt, "\n"), $($arg)*));
}
