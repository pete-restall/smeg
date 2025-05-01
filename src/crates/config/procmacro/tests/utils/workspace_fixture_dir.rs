use std::sync::OnceLock;

pub fn cargo_workspace_dir_slash() -> &'static str {
    static CARGO_WORKSPACE_DIR_SLASH: OnceLock<String> = OnceLock::new();
    CARGO_WORKSPACE_DIR_SLASH.get_or_init(|| format!("{}/", cargo_workspace_dir()))
}

pub fn cargo_workspace_dir() -> &'static str {
    static CARGO_WORKSPACE_DIR: OnceLock<String> = OnceLock::new();
    CARGO_WORKSPACE_DIR.get_or_init(|| env!("CARGO_WORKSPACE_DIR").trim().trim_end_matches('/').to_string())
}

pub fn fixture_dir_for(fixture: &str) -> String {
    format!("{}/crates/config/procmacro/tests/fixtures/{fixture}", cargo_workspace_dir())
}
