pub fn main() {
    smeg_config_procmacro::all_config_filenames!()
        .iter()
        .for_each(|config_filename| println!("cargo:rerun-if-changed={}", config_filename));
}
