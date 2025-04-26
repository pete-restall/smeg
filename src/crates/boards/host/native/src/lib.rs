#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(not(test))]
pub use smeg_mcu_host_native::main;

#[cfg(test)]
pub mod tests {
	#[test]
	pub fn an_example_test() {
		// TODO: An example test to prove it works - remove when doing this for real...
    	assert_eq!(42, 42);
	}
}
