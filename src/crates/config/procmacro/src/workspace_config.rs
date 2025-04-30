pub(crate) struct WorkspaceConfig {
    dir: String
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            dir: sanitised_workspace_dir(CARGO_WORKSPACE_DIR)
        }
    }
}

const CARGO_WORKSPACE_DIR: &str = env!("CARGO_WORKSPACE_DIR");

fn sanitised_workspace_dir(dir: &str) -> String {
    let workspace_dir = sanitised_dir(CARGO_WORKSPACE_DIR);
    sanitised_dir(&dir.replace("${workspace_dir}", &workspace_dir))
}

fn sanitised_dir(dir: &str) -> String {
    dir.trim().trim_end_matches('/').to_string()
}

impl WorkspaceConfig {
    pub fn dir(&self) -> Result<&String, String> {
        if self.dir.is_empty() {
            Err("\
                No workspace_dir argument passed and no CARGO_WORKSPACE_DIR environment variable defined; the latter should be \
                pre-defined in the workspace's .cargo/config.toml and you need to be running the build via cargo".to_string())
        } else {
            Ok(&self.dir)
        }
    }

    pub fn parse_mut(&mut self, args: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if args.path.is_ident("workspace_dir") {
            self.dir = sanitised_workspace_dir(&args.value()?.parse::<syn::LitStr>()?.value());
            Ok(())
        } else {
            Err(args.error("unknown argument for workspace config"))
        }
    }
}
