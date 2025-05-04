use smeg_build_utils::results::StringError;
use crate::workspace_config::WorkspaceConfig;

pub struct MacroArgs {
    workspace: WorkspaceConfig
}

impl MacroArgs {
    pub fn default() -> Self {
        Self {
            workspace: WorkspaceConfig::default()
        }
    }

    pub fn workspace_dir(&self) -> Result<&str, StringError> {
        self.workspace.dir()
    }

    pub fn parse_mut(&mut self, args: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        match args.path.get_ident() {
            Some(id) if id.to_string().starts_with("workspace_") => self.workspace.parse_mut(args),
            _ => Err(args.error("unknown argument"))
        }
    }
}
