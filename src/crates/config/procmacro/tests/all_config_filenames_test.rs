#![allow(non_snake_case)]

use std::sync::OnceLock;

use fluent_test::prelude::{AndModifier, CollectionMatchers, expect, NotModifier};
use smeg_config_procmacro::all_config_filenames;

#[test]
fn all_config_filenames__called_without_workspace_dir__expect_workspace_dir_comes_from_cargo_environment_variable() {
    let discovered_filenames = all_config_filenames!();
    let prefix_matches: Vec<_> = discovered_filenames
        .iter()
        .map(|filename| filename.starts_with(cargo_workspace_dir_slash()))
        .collect();

    expect!(prefix_matches)
        .not().to_be_empty()
        .and().not().to_contain(false);
}

fn cargo_workspace_dir_slash() -> &'static str {
    static CARGO_WORKSPACE_DIR_SLASH: OnceLock<String> = OnceLock::new();
    CARGO_WORKSPACE_DIR_SLASH.get_or_init(|| format!("{}/", cargo_workspace_dir()))
}

fn cargo_workspace_dir() -> &'static str {
    static CARGO_WORKSPACE_DIR: OnceLock<String> = OnceLock::new();
    CARGO_WORKSPACE_DIR.get_or_init(|| env!("CARGO_WORKSPACE_DIR").trim().trim_end_matches('/').to_string())
}

#[test]
fn all_config_filenames__called_with_workspace_dir__expect_workspace_dir_comes_from_cargo_environment_variable() {
    let discovered_filenames = all_config_filenames!(workspace_dir = "crates/config/procmacro/tests/fixtures/single_config");
    let expected_filename = format!("{}/crates/config/procmacro/tests/fixtures/single_config/smeg_config.toml", cargo_workspace_dir());
    expect!(&discovered_filenames).to_equal_collection(&[expected_filename]);
}
