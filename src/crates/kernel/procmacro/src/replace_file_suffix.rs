use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, Token, parse_macro_input};
use syn::parse::{Parse, ParseStream};

struct MacroArgs {
    old_suffix: LitStr,
    _1: Token![,],
    new_suffix: LitStr
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            old_suffix: input.parse()?,
            _1: input.parse()?,
            new_suffix: input.parse()?
        })
    }
}

pub fn replace_file_suffix(items: TokenStream) -> TokenStream {
    let args = parse_macro_input!(items as MacroArgs);
    let source_path = super::source_path_of_macro_invocation();
    if let Some(with_new_suffix) = super::try_replace_suffix(&source_path, &args.old_suffix.value(), &args.new_suffix.value()) {
        quote! { #with_new_suffix }.into()
    }
    else
    {
        quote! { #source_path }.into()
    }
}
