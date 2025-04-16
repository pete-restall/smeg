pub fn main() {
	let source_linker_script =
		std::env::var("SMEG_SOURCE_LINKER_SCRIPT")
		.expect("SMEG_SOURCE_LINKER_SCRIPT is not defined; are you building outside of build.sh ?");

	let target_linker_script =
		std::env::var("SMEG_TARGET_LINKER_SCRIPT")
		.expect("SMEG_TARGET_LINKER_SCRIPT is not defined; are you building outside of build.sh ?");

	println!("cargo:rerun-if-changed={}", &source_linker_script);
	println!("cargo:rerun-if-changed={}", &target_linker_script);
	println!("cargo:rustc-link-arg=-T{}", &target_linker_script);
}
