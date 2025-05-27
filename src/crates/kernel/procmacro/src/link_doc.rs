use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
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

pub fn link_doc(args: TokenStream, items: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as MacroArgs);
    let items = TokenStream2::from(items);
    let source_path = super::source_path_of_macro_invocation();
    if let Some(anchor) = args.anchor {
        let md_path_and_anchor = rs_to_md_path_with_anchor(&source_path, &anchor.value());
        quote! {
            #[cfg_attr(doc, doc = ::include_utils::include_md!(#md_path_and_anchor))]
            #items
        }
    } else {
        let md_path_and_anchor = rs_to_md_path_with_anchor(&source_path, "summary");
        quote! {
            //!
            ::include_utils::include_md!(#md_path_and_anchor)
            //!
            #items
        }
    }.into()
}

fn rs_to_md_path_with_anchor(source_filename: &str, anchor: &str) -> String {
    super::
        try_replace_suffix(source_filename, ".rs", &format!(".md:{anchor}"))
        .unwrap_or_else(|| panic!("Expected macro invocation from filename with Rust suffix of '.rs'; filename={source_filename}"))
}
