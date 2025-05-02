use proc_macro::TokenStream;

use super::config_file_discovery::workspace_config_filenames_in;

mod all_config;

pub fn populate_from_config(args: TokenStream, items: TokenStream) -> TokenStream {
    let mut parsed_args = PopulateFromConfigArgs::default();
    let args_parser = syn::meta::parser(|args| parsed_args.parse_mut(args));
    syn::parse_macro_input!(args with args_parser);

    let parsed_items = syn::parse_macro_input!(items);

    let (default_config_filenames, override_config_filename) = workspace_config_filenames_in(parsed_args.workspace.dir().unwrap()).unwrap();
    all_config::load_from(
        &default_config_filenames,
        &override_config_filename,
        &parsed_items).unwrap().into()
}

struct PopulateFromConfigArgs {
    workspace: super::WorkspaceConfig
}

impl PopulateFromConfigArgs {
    fn default() -> Self {
        Self {
            workspace: super::WorkspaceConfig::default()
        }
    }

    fn parse_mut(&mut self, args: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        match args.path.get_ident() {
            Some(id) if id.to_string().starts_with("workspace_") => self.workspace.parse_mut(args),
            _ => Err(args.error("unknown argument"))
        }
    }
}
