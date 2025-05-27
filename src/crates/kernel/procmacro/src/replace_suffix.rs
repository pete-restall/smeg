use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, Token, parse_macro_input};
use syn::parse::{Parse, ParseStream};

struct MacroArgs {
    string: LitStr,
    _1: Token![,],
    old_suffix: LitStr,
    _2: Token![,],
    new_suffix: LitStr
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            string: input.parse()?,
            _1: input.parse()?,
            old_suffix: input.parse()?,
            _2: input.parse()?,
            new_suffix: input.parse()?
        })
    }
}

pub fn replace_suffix(items: TokenStream) -> TokenStream {
    let args = parse_macro_input!(items as MacroArgs);
    let string = args.string.value();
    if let Some(with_new_suffix) = try_replace_suffix(&string, &args.old_suffix.value(), &args.new_suffix.value()) {
        quote! { #with_new_suffix }.into()
    } else {
        quote! { #string }.into()
    }
}

pub(crate) fn try_replace_suffix(string: &str, old_suffix: &str, new_suffix: &str) -> Option<String> {
    string.strip_suffix(old_suffix).map(|without_suffix| format!("{without_suffix}{new_suffix}"))
}
