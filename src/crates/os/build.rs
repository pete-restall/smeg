pub fn main() {
    let source_linker_script = std::env::var("SMEG_SOURCE_LINKER_SCRIPT");
    let target_linker_script = std::env::var("SMEG_TARGET_LINKER_SCRIPT");
    rerun_if_changed(&[
        &source_linker_script,
        &target_linker_script]);

    if let Ok(filename) = target_linker_script {
        println!("cargo:rustc-link-arg=-T{}", filename);
    }

    let link_with_libc = as_boolean(std::env::var("SMEG_LINK_WITH_LIBC"));
    if link_with_libc {
        println!("cargo:rustc-link-arg=-lc");
    }
}

fn rerun_if_changed<T>(filenames: &[&Result<String, T>]) {
    for filename in filenames {
        if let Ok(filename) = *filename {
            println!("cargo:rerun-if-changed={}", filename);
        }
    }
}

fn as_boolean<T>(value: Result<String, T>) -> bool {
    if let Ok(possibly_boolean) = value {
        if let Ok(as_boolean) = possibly_boolean.parse::<bool>() {
            return as_boolean;
        }
    }
    false
}
