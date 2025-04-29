use proc_macro::TokenStream;

mod all_config;
mod all_config_filenames_macro;
mod config_file_discovery;
mod populate_from_config_macro;
mod results;

mod workspace_config;
pub(crate) type WorkspaceConfig = workspace_config::WorkspaceConfig;

#[proc_macro]
pub fn all_config_filenames(items: TokenStream) -> TokenStream {
    all_config_filenames_macro::all_config_filenames(items)
}

#[proc_macro_attribute]
pub fn populate_from_config(args: TokenStream, items: TokenStream) -> TokenStream {
    populate_from_config_macro::populate_from_config(args, items)
}
