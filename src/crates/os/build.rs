use std::io::Write;

use smeg_build_utils::smeg_out_dir;
use smeg_build_utils::results::ResultAnyError;

pub fn main() -> ResultAnyError<()> {
    let smeg_out_dir = smeg_out_dir()?;

    let link_with_libc = as_boolean(std::env::var("SMEG_LINK_WITH_LIBC"));
    if link_with_libc {
        println!("cargo:rustc-link-arg=-lc");
    }

    let linker_script_filename = format!("{smeg_out_dir}/linker-script.lld");
    if std::fs::exists(&linker_script_filename)? {
        println!("cargo::rustc-link-arg=-T{linker_script_filename}");
    }

    let mut fd = std::fs::File::create(format!("{smeg_out_dir}/smeg_config_dump.txt"))?;
    write!(&mut fd, "{:#?}", smeg_config::SMEG_CONFIG)?;

    Ok(())
}

fn as_boolean<T>(value: Result<String, T>) -> bool {
    if let Ok(possibly_boolean) = value {
        if let Ok(as_boolean) = possibly_boolean.parse::<bool>() {
            return as_boolean;
        }
    }
    false
}
