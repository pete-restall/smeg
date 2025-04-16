fn main() {
//    let linkScript = env::get("OUT_DIR") + "/stm32l432kc.out.ld";
// transform the ld as a template ?
// we want two linker scripts to be generated - one for the kernel, one for the application
// we can tailor the linker script depending on which features have been enabled (eg. if RAM can be put into power-saving, then the kernel needs to be put into the always-on RAM; this same feature could be used as a toggle to link in (or exclude) that code, so there was no chance of calling a function accidentally that put the RAM to sleep when the kernel could not handle it)
//    println!("cargo::rustc-link-arg-bin=-T{0}", linkScript);
}
