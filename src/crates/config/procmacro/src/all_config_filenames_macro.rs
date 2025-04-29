use proc_macro::TokenStream;

use quote::quote;

use super::config_file_discovery::workspace_config_filenames_in;

pub(crate) fn all_config_filenames(items: TokenStream) -> TokenStream {
    let mut parsed_args = AllConfigFilenamesArgs::default();
    let args_parser = syn::meta::parser(|args| parsed_args.parse_mut(args));
    syn::parse_macro_input!(items with args_parser);

    let (default_config_filenames, override_config_filename) = workspace_config_filenames_in(parsed_args.workspace.dir().unwrap()).unwrap();
    quote! {
        [#( #default_config_filenames, )* #override_config_filename]
    }.into()
}

struct AllConfigFilenamesArgs {
    workspace: super::WorkspaceConfig
}

impl AllConfigFilenamesArgs {
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
