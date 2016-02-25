#![allow(non_camel_case_types)]
#![allow(unused_attributes)]

// Detect 32-bit platforms
#![cfg_attr(
	any(
		target_arch = "x86",
		target_arch = "arm",
		target_arch = "mips",
		target_arch = "mipsel",
		target_arch = "powerpc",
		target_arch = "le32"
	),
	target_arch_std32
)]

// Detect 64-bit platforms
#![cfg_attr(
	any(
		target_arch = "x86_64",
		target_arch = "aarch64"
	),
	target_arch_std64
)]

// Special: Platforms with unsigned character types
#![cfg_attr(
	any(
		target_arch = "aarch64"
	),
	target_arch_char_unsigned
)]

// Character types
#[cfg(not(target_arch_char_unsigned))]
pub type c_char  = i8;
#[cfg(target_arch_char_unsigned)]
pub type c_char  = u8;
#[cfg(not(target_arch_char_unsigned))]
pub type c_wchar = i32;
#[cfg(target_arch_char_unsigned)]
pub type c_wchar = u32;

// Standard integers
pub type c_int = i32;

// The special "size" type
#[cfg(target_arch_std32)]
pub type size_t = u32;
#[cfg(target_arch_std64)]
pub type size_t = u64;
