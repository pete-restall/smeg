use proc_macro::TokenStream;
use syn::DeriveInput;

use super::{RegisterDefinition, Single};

macro_rules! generate_token_stream_for {
    (&$derive:ident, [ $($types:ident),+ ]) => {
        match register_type_name_from(&$derive.data) {
            $(
                Some((stringify!($types), type_ident)) =>
                    RegisterDefinition::<$types>::parse(&$derive, &type_ident).map(|x| x.generate()),
            )+

            _ => Err(concat!("A register can only be defined as struct(", stringify!($($types)|+), ")").to_string())
        }.unwrap()
    }
}

macro_rules! match_register_type {
    ($field:ident.ty, [ $($types:ident),+ ]) => {
        if let ::syn::Type::Path(ref path) = $field.ty && let Some(ident) = path.path.get_ident() {
            match ($field.ident.is_none(), ident.to_string().as_str()) {
                $((true, stringify!($types)) => Some((
                    stringify!($types),
                    ::syn::Ident::new(stringify!($types), ::proc_macro2::Span::call_site()))),)+

                _ => None
            }
        } else {
            None
        }
    };
}

fn register_type_name_from(struct_body: &syn::Data) -> Option<(&'static str, syn::Ident)> {
    if
        let syn::Data::Struct(register_struct) = struct_body &&
        let Ok(field) = register_struct.fields.iter().single() {

        return match_register_type!(field.ty, [i8, u8, i16, u16, i32, u32, i64, u64, isize, usize]);
    }

    None
}

pub fn mmio_register(_args: TokenStream, items: TokenStream) -> TokenStream {
    let derive = syn::parse_macro_input!(items as DeriveInput);
    generate_token_stream_for!(&derive, [i8, u8, i16, u16, i32, u32, i64, u64, isize, usize]).into()
}
