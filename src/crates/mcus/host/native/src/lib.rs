#![cfg_attr(not(test), no_std)]

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
	// TODO: this will have to call into the smeg-os entrypoint just like the MCU bootstrappers...
	0
}
