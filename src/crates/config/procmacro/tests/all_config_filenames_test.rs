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
    let discovered_filenames = all_config_filenames!(workspace_dir = "${workspace_dir}/crates/config/procmacro/tests/fixtures/single_config");
    let expected_filename = format!("{}/smeg_config.toml", fixture_dir_for("single_config"));
    expect!(&discovered_filenames).to_equal_collection(&[expected_filename]);
}

fn fixture_dir_for(fixture: &str) -> String {
    format!("{}/crates/config/procmacro/tests/fixtures/{fixture}", cargo_workspace_dir())
}

#[test]
fn all_config_filenames__called_when_non_smeg_config_tomls_exist__expect_only_smeg_config_files_are_included() {
    let discovered_filenames = all_config_filenames!(workspace_dir = "${workspace_dir}/crates/config/procmacro/tests/fixtures/superfluous_config");
    let fixture_dir = fixture_dir_for("superfluous_config");
    expect!(&discovered_filenames).to_equal_collection(&[
        format!("{fixture_dir}/crates/kernel/more_nesting/smeg_config.toml"),
        format!("{fixture_dir}/smeg_config.toml")
    ]);
}

#[test]
fn all_config_filenames__called_when_hierarchical_smeg_config_tomls_exist__expect_config_files_are_ordered_from_most_general_to_most_specific() {
    let discovered_filenames = all_config_filenames!(workspace_dir = "${workspace_dir}/crates/config/procmacro/tests/fixtures/simple_hierarchical_config");
    let fixture_dir = fixture_dir_for("simple_hierarchical_config");
    expect!(&discovered_filenames).to_equal_collection(&[
        format!("{fixture_dir}/crates/kernel/smeg_config.toml"),
        format!("{fixture_dir}/crates/mcus/b_family/smeg_config.toml"),
        format!("{fixture_dir}/crates/mcus/a/smeg_config.toml"),
        format!("{fixture_dir}/crates/drivers/abc_family/smeg_config.toml"),
        format!("{fixture_dir}/crates/drivers/abc/smeg_config.toml"),
        format!("{fixture_dir}/crates/boards/123_family/smeg_config.toml"),
        format!("{fixture_dir}/crates/boards/0/smeg_config.toml"),
        format!("{fixture_dir}/smeg_config.toml")
    ]);
}
