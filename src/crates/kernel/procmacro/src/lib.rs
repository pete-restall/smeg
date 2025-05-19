use proc_macro::TokenStream;

mod error_tag;

#[proc_macro]
pub fn error_tag(items: TokenStream) -> TokenStream {
    error_tag::error_tag(items)
}
