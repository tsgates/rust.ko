#![allow(non_camel_case_types)]
#![allow(unused_attributes)]

// Special: Platforms with unsigned character types
#![cfg_attr(
	any(
		target_arch = "aarch64",
		target_arch = "arm",
		target_arch = "powerpc",
		target_arch = "powerpc64"
	),
	target_arch_char_unsigned
)]

// Character types
pub type c_schar = i8;
pub type c_uchar = u8;
#[cfg(not(target_arch_char_unsigned))]
pub type c_char  = i8;
#[cfg(target_arch_char_unsigned)]
pub type c_char  = u8;
#[cfg(not(target_arch_char_unsigned))]
pub type c_wchar = i32;
#[cfg(target_arch_char_unsigned)]
pub type c_wchar = u32;

// Standard integers
pub type c_short     = i16;
pub type c_ushort    = u16;

pub type c_int       = i32;
pub type c_uint      = u32;

#[cfg(target_pointer_width = "32")]
pub type c_long      = i32;
#[cfg(target_pointer_width = "32")]
pub type c_ulong     = u32;
#[cfg(target_pointer_width = "64")]
pub type c_long      = i64;
#[cfg(target_pointer_width = "64")]
pub type c_ulong     = u64;

pub type c_longlong  = i64;
pub type c_ulonglong = u64;

// Float point type (not usable, but they do exist)
pub type c_float  = f32;
pub type c_double = f64;

// The special "size" type
#[cfg(target_pointer_width = "32")]
pub type c_size_t = u32;
#[cfg(target_pointer_width = "64")]
pub type c_size_t = u64;

/// Magic pointer that represents a `void*` in C
/// See `libstd/os/raw.rs:48` from the Rust source code for details…
#[repr(u8)]
pub enum c_void {
	#[doc(hidden)] __variant1,
	#[doc(hidden)] __variant2
}