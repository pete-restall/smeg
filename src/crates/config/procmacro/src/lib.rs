use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod config_file_discovery;
use config_file_discovery::workspace_config_filenames;

mod all_config;

#[proc_macro_attribute]
pub fn populate_from_config(_args: TokenStream, items: TokenStream) -> TokenStream {
    let (default_config_filenames, override_config_filename) = workspace_config_filenames().unwrap();
    let parsed = parse_macro_input!(items);
    all_config::load_from(
        &default_config_filenames,
        &override_config_filename,
        &parsed).unwrap().into()
}

#[proc_macro]
pub fn all_config_filenames(_items: TokenStream) -> TokenStream {
    let (default_config_filenames, override_config_filename) = workspace_config_filenames().unwrap();
    quote! {
        [#( #default_config_filenames, )* #override_config_filename]
    }.into()
}
