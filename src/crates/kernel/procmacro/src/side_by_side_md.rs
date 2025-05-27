use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};
use syn::parse::Parse;

struct MacroArgs {
    anchor: Option<LitStr>
}

impl Parse for MacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            anchor: input.parse()?
        })
    }
}

pub fn side_by_side_md(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as MacroArgs);
    let source_path = super::source_path_of_macro_invocation();
    let md_path_and_anchor = rs_to_md_path_with_anchor(
        &source_path,
        &args.anchor.map_or_else(|| "module".to_string(), |a| a.value()));

    quote! {
        ::include_utils::include_md!(#md_path_and_anchor)
    }.into()
}

fn rs_to_md_path_with_anchor(source_filename: &str, anchor: &str) -> String {
    super::
        try_replace_suffix(source_filename, ".rs", &format!(".md:{anchor}"))
        .unwrap_or_else(|| panic!("Expected macro invocation from filename with Rust suffix of '.rs'; filename={source_filename}"))
}
