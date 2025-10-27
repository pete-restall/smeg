#![feature(proc_macro_span)]

use proc_macro::{Span, TokenStream};

mod error_tag;

#[proc_macro]
pub fn error_tag(items: TokenStream) -> TokenStream {
    error_tag::error_tag(items)
}

mod mmio;

#[proc_macro_attribute]
pub fn mmio_register(args: TokenStream, input: TokenStream) -> TokenStream {
    mmio::mmio_register(args, input)
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

mod side_by_side_md;

#[proc_macro]
pub fn side_by_side_md(items: TokenStream) -> TokenStream {
    side_by_side_md::side_by_side_md(items)
}

pub(crate) fn source_path_of_macro_invocation() -> String {
    Span::call_site().file()
}

pub(crate) use replace_suffix::try_replace_suffix;
