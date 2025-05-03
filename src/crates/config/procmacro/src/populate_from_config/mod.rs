use proc_macro::TokenStream;

use crate::config_file_discovery::workspace_config_filenames_in;

mod all_config;

mod macro_args;
use macro_args::MacroArgs;

pub fn populate_from_config(args: TokenStream, items: TokenStream) -> TokenStream {
    let mut parsed_args = MacroArgs::default();
    let args_parser = syn::meta::parser(|args| parsed_args.parse_mut(args));
    syn::parse_macro_input!(args with args_parser);

    let parsed_items = syn::parse_macro_input!(items);
    let workspace_dir = parsed_args.workspace_dir().unwrap();
    let (default_config_filenames, override_config_filename) = workspace_config_filenames_in(workspace_dir).unwrap();
    all_config::load_from(
        &default_config_filenames,
        &override_config_filename,
        &parsed_items).unwrap().into()
}
