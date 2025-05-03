use proc_macro::TokenStream;
use quote::quote;

use crate::config_file_discovery::workspace_config_filenames_in;

mod macro_args;
use macro_args::MacroArgs;

pub fn all_config_filenames(items: TokenStream) -> TokenStream {
    let mut parsed_args = MacroArgs::default();
    let args_parser = syn::meta::parser(|args| parsed_args.parse_mut(args));
    syn::parse_macro_input!(items with args_parser);

    let workspace_dir = parsed_args.workspace_dir().unwrap();
    let (default_config_filenames, override_config_filename) = workspace_config_filenames_in(workspace_dir).unwrap();
    quote! {
        [#( #default_config_filenames, )* #override_config_filename]
    }.into()
}
