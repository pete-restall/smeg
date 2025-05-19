use proc_macro::TokenStream;
use quote::quote;

pub fn error_tag(items: TokenStream) -> TokenStream {
    let details = proc_macro2::TokenStream::from(items).into_iter();
    quote! {
        {
            use ::core::mem::MaybeUninit;

            use ::smeg_kernel::errors::ErrorTag;

            #[unsafe(link_section = ".smeg.tags.errors")]
            #[unsafe(export_name = concat!(module_path!(), "\t", file!(), "\t", line!(), "\t", #( #details )*))]
            static TAG: MaybeUninit<u8> = MaybeUninit::<u8>::new(0);
            ErrorTag::new(&TAG)
        }
    }.into()
}
