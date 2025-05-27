#![feature(proc_macro_span)]

use proc_macro::{Span, TokenStream};

mod error_tag;

#[proc_macro]
pub fn error_tag(items: TokenStream) -> TokenStream {
    error_tag::error_tag(items)
}

mod link_doc;

#[proc_macro_attribute]
pub fn link_doc(args: TokenStream, items: TokenStream) -> TokenStream {
    link_doc::link_doc(args, items)
}

mod replace_file_suffix;

#[proc_macro]
pub fn replace_file_suffix(items: TokenStream) -> TokenStream {
    replace_file_suffix::replace_file_suffix(items)
}

mod replace_suffix;

#[proc_macro]
pub fn replace_suffix(items: TokenStream) -> TokenStream {
    replace_suffix::replace_suffix(items)
}

pub(crate) fn source_path_of_macro_invocation() -> String {
    let source_path = Span::call_site()
        .source()
        .source_file()
        .path();

    source_path
        .to_str()
        .expect("Source path (ie. file!()) contains non-UTF-8 characters")
        .to_string()
}

pub(crate) use replace_suffix::try_replace_suffix;
