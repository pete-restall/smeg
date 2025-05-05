use std::io::Write;

use serde::Serialize;
use tinytemplate::TinyTemplate;

use smeg_build_utils::smeg_out_dir;
use smeg_build_utils::results::*;
use smeg_config::SMEG_CONFIG;

static LINKER_SCRIPT_TEMPLATE: &str = include_str!("linker-script.lld.tt");

fn main() -> ResultAnyError<()> {
    let target_triple = std::env::var("TARGET").unwrap_or_default();
    let host_triple = std::env::var("HOST").unwrap_or_default();
    if target_triple == host_triple {
        println!("Not compiling for an MCU target, so build script has nothing further to do.");
        return Ok(());
    }

    let smeg_out_dir = smeg_out_dir()?;
    let linker_script_filename = format!("{smeg_out_dir}/smeg-os.lld");

    let mut fd = std::fs::File::create(&linker_script_filename)?;
    fd.write_all(create_linker_script_from(LINKER_SCRIPT_TEMPLATE)?.as_bytes())?;

    println!("cargo:rerun-if-changed=linker-script.lld.tt");
    println!("cargo:rerun-if-changed={linker_script_filename}");

    Ok(())
}

fn create_linker_script_from(template: &str) -> ResultAnyError<String> {
    #[derive(Serialize)]
    struct Context {
        sram2_size_in_words: usize
    }

    let mut tt = TinyTemplate::new();
    tt.add_template("linker-script", template)?;

    let context = Context {
        sram2_size_in_words: (
            SMEG_CONFIG.VALUES.KERNEL.STACK.SIZE_IN_WORDS +
            SMEG_CONFIG.VALUES.MCUS.ST.STM32L432KC.RESERVE_SRAM2_WORDS) as usize
    };

    Ok(tt.render("linker-script", &context)?)
}
