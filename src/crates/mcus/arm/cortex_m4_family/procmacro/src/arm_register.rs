use proc_macro::TokenStream;
use syn::DeriveInput;

use crate::Single;

use super::register_definition::RegisterDefinitionGenerator;

// TODO: most of this file can go in the kernel proc macro crate; maybe the 'arm_register' below calls that with a list of allowed types (ie. '[iu]32' and '[iu]size')

pub fn arm_register(_args: TokenStream, items: TokenStream) -> TokenStream {
    let derive = syn::parse_macro_input!(items as DeriveInput);
    match register_type_name_from(&derive.data) {
        Some(("i8", type_ident)) => RegisterDefinitionGenerator::<i8>::generate(&derive, &type_ident),
        Some(("u8", type_ident)) => RegisterDefinitionGenerator::<u8>::generate(&derive, &type_ident),

        Some(("i16", type_ident)) => RegisterDefinitionGenerator::<i16>::generate(&derive, &type_ident),
        Some(("u16", type_ident)) => RegisterDefinitionGenerator::<u16>::generate(&derive, &type_ident),

        Some(("i32", type_ident)) => RegisterDefinitionGenerator::<i32>::generate(&derive, &type_ident),
        Some(("u32", type_ident)) => RegisterDefinitionGenerator::<u32>::generate(&derive, &type_ident),

        Some(("i64", type_ident)) => RegisterDefinitionGenerator::<i64>::generate(&derive, &type_ident),
        Some(("u64", type_ident)) => RegisterDefinitionGenerator::<u64>::generate(&derive, &type_ident),

        Some(("isize", type_ident)) => RegisterDefinitionGenerator::<isize>::generate(&derive, &type_ident),
        Some(("usize", type_ident)) => RegisterDefinitionGenerator::<usize>::generate(&derive, &type_ident),

        _ => Err("A register can only be defined as struct(i8|u8|i16|u16|i32|u32|i64|u64|isize|usize)".to_string())
    }.unwrap().into()
}

macro_rules! match_register_type {
    ($field:ident.ty { $($types:ident),+ }) => {
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

        return match_register_type! {
            field.ty { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize }
        };
    }

    None
}
