use proc_macro::TokenStream;

mod all_config_filenames;
mod config_file_discovery;
mod populate_from_config;
mod workspace_config;

#[proc_macro]
pub fn all_config_filenames(items: TokenStream) -> TokenStream {
    all_config_filenames::all_config_filenames(items)
}

#[proc_macro_attribute]
pub fn populate_from_config(args: TokenStream, items: TokenStream) -> TokenStream {
    populate_from_config::populate_from_config(args, items)
}
