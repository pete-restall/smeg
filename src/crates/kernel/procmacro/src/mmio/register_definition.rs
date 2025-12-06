use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Ident, Visibility};

use super::*;

pub struct RegisterDefinition<T>
    where
        T:
            BitAnd<Output = T> +
            BitOr<Output = T> +
            BitXor<Output = T> +
            Copy +
            Debug +
            Default +
            Display +
            FromStr +
            Not<Output = T> +
            PartialEq,
        T::Err: Display {

    type_ident: Ident,
    struct_visibility: Visibility,
    struct_ident: Ident,
    all_bits_set: T,
    fields: Vec<RegisterFieldDefinition<T>>
}

impl<T> RegisterDefinition<T>
    where
        T:
            BitAnd<Output = T> +
            BitOr<Output = T> +
            BitXor<Output = T> +
            Copy +
            Debug +
            Default +
            Display +
            FromStr +
            Not<Output = T> +
            PartialEq +
            Into<RegisterFieldMask<T>> +
            ToTokens,
        T::Err: Display,
        RegisterFieldMask<T>: RegisterFieldMaskProperties<MaskType = T> {

    pub fn try_parse(derive: &DeriveInput, type_ident: Ident) -> Result<Self, String> {
        let attributes = derive.attrs.iter().map(RegisterAttribute::<T>::try_parse).collect::<Result<Vec<_>, _>>()?;

        let datasheet = Self::parse_datasheet_from(&attributes)?;
        let fields = Self::parse_fields_from(&attributes)?;

        let all_bits_set = !T::default();
        let mask = Self::mask_for(&fields)?;
        if mask != all_bits_set {
            return Err("Register definition has incomplete field mask; every bit needs to be defined".to_string())
        }

        Ok(Self {
            struct_visibility: derive.vis.clone(),
            struct_ident: derive.ident.clone(),
            all_bits_set,
            fields: fields.iter().map(|f| RegisterFieldDefinition::try_parse((*f).clone(), type_ident.clone())).collect::<Result<Vec<_>, _>>()?,
            type_ident
        })
    }

    fn parse_datasheet_from(attributes: &[RegisterAttribute<T>]) -> Result<Option<&RegisterDatasheetAttribute>, &str> {
        attributes
            .iter()
            .filter_map(|attr| match attr { RegisterAttribute::Datasheet(datasheet) => Some(datasheet), _ => None })
            .single_or_none()
            .map_err(|_| "Register definition cannot have multiple datasheet attributes specified")
    }

    fn parse_fields_from(attributes: &[RegisterAttribute<T>]) -> Result<Vec<&RegisterFieldAttribute<T>>, &str> {
        let mut fields: HashMap<&str, &RegisterFieldAttribute<T>> = HashMap::default();
        let are_all_fields_unique = attributes
            .iter()
            .filter_map(|attr| match attr { RegisterAttribute::Field(field) => Some(field), _ => None })
            .filter_map(|field| fields.insert(field.name_uppercase(), field))
            .next()
            .is_none();

        if are_all_fields_unique {
            Ok(fields.into_values().collect())
        } else {
            Err("Register definition has duplicate field names")
        }
    }

    fn mask_for(fields: &[&RegisterFieldAttribute<T>]) -> Result<T, &'static str> {
        let zero = T::default();
        fields.iter().try_fold(zero, |mask, field| if (mask & field.mask().value()) == zero {
            Ok(mask | field.mask().value())
        } else {
            Err("Register definition has overlapping field masks")
        })
    }

    pub fn generate(&self) -> TokenStream {
        let (type_ident, visibility, register_ident) = (&self.type_ident, &self.struct_visibility, &self.struct_ident);
        let field_definitions = &self.fields;
        quote! {
            #[repr(transparent)]
            #visibility struct #register_ident(#type_ident);

            unsafe impl ::smeg_kernel::mem::CellPrimitive for #register_ident {
                type Type = #type_ident;
            }

            impl #register_ident {
                #(#field_definitions)*

                // also want some other const booleans - IS_READABLE, IS_WRITABLE, IS_READONLY, IS_WRITEONLY - but put these on a trait
                pub const IS_READABLE: bool = true; // TODO

                pub const IS_WRITABLE: bool = false; // TODO

                pub const IS_READONLY: bool = Self::IS_READABLE && !Self::IS_WRITABLE;

                pub const IS_WRITEONLY: bool = !Self::IS_READABLE && Self::IS_WRITABLE;

                pub const HAS_RESERVED_BITS: bool = false;
                pub const RESERVED_MASK: #type_ident = 0;
                pub const RESERVED_UNK_SBZ_MASK: #type_ident = 0;
                pub const RESERVED_UNK_SBZP_MASK: #type_ident = 0;
                pub const RESERVED_UNK_SBO_MASK: #type_ident = 0;
                pub const RESERVED_UNK_SBOP_MASK: #type_ident = 0;
                pub const RESERVED_UNK_SBP_MASK: #type_ident = 0;
                pub const RESERVED_SBZ_MASK: #type_ident = 0;
                pub const RESERVED_SBZP_MASK: #type_ident = 0;
                pub const RESERVED_SBO_MASK: #type_ident = 0;
                pub const RESERVED_SBOP_MASK: #type_ident = 0;
                pub const RESERVED_SBP_MASK: #type_ident = 0;
                pub const RESERVED_WI_MASK: #type_ident = 0;

            }

    //        impl<'mem> #accessor_name<'mem> {
                // TODO...can't use generics from 'input'...
    //    pub fn implementer(&self) -> usize { unsafe { self.accessor.mmio_read_masked_shifted_right::<{Cpuid::IMPLEMENTER_MASK}>() } }
    //     pub fn implementer_raw(&self) -> usize { unsafe { self.accessor.mmio_read_masked::<{Cpuid::IMPLEMENTER_MASK}>() } }

            // TODO: #visibility type {#register_name}Cell<M> = {Readonly|Writeonly|ReadWrite}Cell<M, #register_name>;
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use proc_macro2::Span;
    use syn::parse_quote;

    use fluent_test::prelude::*;

    use smeg_testing_host_utils::integers::*;
    use smeg_testing_host_utils::strings::AnyCase;
    use smeg_testing_host_utils::strings::ascii::any_rust_identifier;

    use super::*;

    #[test]
    fn try_parse__called_when_multiple_datasheets_given__expect_err() {
        let too_many_datasheets: DeriveInput = parse_quote! {
            #[datasheet("somewhere", "section", 123)]
            #[datasheet("elsewhere", "another", 456)]
            #[ro(FIELD_1, 0xff)]
            struct DummyRegister(u8);
        };

        let u8_ident = Ident::new("u8", Span::call_site());
        let result = RegisterDefinition::<u8>::try_parse(&too_many_datasheets, u8_ident);

        let error = result.map(|_| ()).unwrap_err();
        expect!(error).to_contain("multiple datasheet");
    }

    #[test]
    fn try_parse__called_when_field_names_are_not_case_insentitively_unique__expect_err() {
        let (field_a_name, mask_a) = (any_rust_identifier(), any_usize_except_in(&[0, !0]));
        let (field_b_name, mask_b) = (field_a_name.any_case(), !mask_a);
        let field_a_ident = Ident::new(&field_a_name, Span::call_site());
        let field_b_ident = Ident::new(&field_b_name, Span::call_site());

        let malformed_attributes: Vec<DeriveInput> = vec![
            parse_quote! {
                #[ro(#field_a_ident, #mask_a)]
                #[ro(#field_b_ident, #mask_b)]
                struct DummyRegister(usize);
            },

            parse_quote! {
                #[ro(#field_a_ident, #mask_a)]
                #[wo(#field_b_ident, #mask_b)]
                struct DummyRegister(usize);
            },

            parse_quote! {
                #[wo(#field_a_ident, #mask_a)]
                #[ro(#field_b_ident, #mask_b)]
                struct DummyRegister(usize);
            },

            parse_quote! {
                #[ro(#field_a_ident, #mask_a)]
                #[rw(#field_b_ident, #mask_b)]
                struct DummyRegister(usize);
            },

            parse_quote! {
                #[wo(#field_a_ident, #mask_a)]
                #[rw(#field_b_ident, #mask_b)]
                struct DummyRegister(usize);
            },

            parse_quote! {
                #[wo(#field_a_ident, #mask_a)]
                #[wo(#field_b_ident, #mask_b)]
                struct DummyRegister(usize);
            },

            parse_quote! {
                #[rw(#field_a_ident, #mask_a)]
                #[rw(#field_b_ident, #mask_b)]
                struct DummyRegister(usize);
            }
        ];

        for malformed_attribute in malformed_attributes {
            let usize_ident = Ident::new("usize", Span::call_site());
            let result = RegisterDefinition::<usize>::try_parse(&malformed_attribute, usize_ident);

            let error = result.map(|_| ()).unwrap_err();
            expect!(error).to_contain("duplicate field");
        }
    }

    #[test]
    fn try_parse__called_when_field_have_overlapping_masks__expect_err() {
        let overlapping_bit = 1 << any_u32_within(0..u32::BITS);
        let mask_a = any_u32_except(0) | overlapping_bit;
        let mask_b = !mask_a | overlapping_bit;

        let field_name = any_rust_identifier();
        let field_a_ident = Ident::new(&format!("{field_name}a"), Span::call_site());
        let field_b_ident = Ident::new(&format!("{field_name}b"), Span::call_site());

        let malformed_attributes: Vec<DeriveInput> = vec![
            parse_quote! {
                #[ro(#field_a_ident, #mask_a)]
                #[ro(#field_b_ident, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[ro(#field_a_ident, #mask_a)]
                #[wo(#field_b_ident, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[wo(#field_a_ident, #mask_a)]
                #[ro(#field_b_ident, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[ro(#field_a_ident, #mask_a)]
                #[rw(#field_b_ident, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[ro(#field_a_ident, #mask_a)]
                #[xx(SBZP, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[wo(#field_a_ident, #mask_a)]
                #[rw(#field_b_ident, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[wo(#field_a_ident, #mask_a)]
                #[wo(#field_b_ident, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[wo(#field_a_ident, #mask_a)]
                #[xx(SBOP, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[rw(#field_a_ident, #mask_a)]
                #[rw(#field_b_ident, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[rw(#field_a_ident, #mask_a)]
                #[xx(SBO, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[xx(SBZ, #mask_a)]
                #[xx(UNK_SBOP, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[ro(RO_FIELD, #mask_a)]
                #[rw(RW_FIELD, #mask_b)]
                #[xx(SBZ, #mask_b)]
                struct DummyRegister(u32);
            },

            parse_quote! {
                #[ro(RO_FIELD, #mask_a)]
                #[wo(WO_FIELD, #mask_b)]
                #[xx(SBZ, #mask_a)]
                struct DummyRegister(u32);
            }
        ];

        for malformed_attribute in malformed_attributes {
            let u32_ident = Ident::new("u32", Span::call_site());
            let result = RegisterDefinition::<u32>::try_parse(&malformed_attribute, u32_ident);

            let error = result.map(|_| ()).unwrap_err();
            expect!(error).to_contain("overlapping field masks");
        }
    }

    #[test]
    fn try_parse__called_when_field_masks_are_zero__expect_err() {
        let no_field_masks: DeriveInput = parse_quote! { struct DummyRegister(u32); };
        let u32_ident = Ident::new("u32", Span::call_site());
        let result = RegisterDefinition::<u32>::try_parse(&no_field_masks, u32_ident);

        let error = result.map(|_| ()).unwrap_err();
        expect!(error).to_contain("incomplete field mask");
    }

    #[test]
    fn try_parse__called_when_field_masks_do_not_cover_every_bit__expect_err() {
        let mut incomplete_masks: Vec<DeriveInput> = Vec::with_capacity(4 * 16);
        for i in 0..16 {
            let mask_missing_bit: u16 = !(1 << i);
            incomplete_masks.extend_from_slice(&[
                parse_quote! { #[ro(FIELD, #mask_missing_bit)] struct DummyRegister(u16); },
                parse_quote! { #[wo(FIELD, #mask_missing_bit)] struct DummyRegister(u16); },
                parse_quote! { #[rw(FIELD, #mask_missing_bit)] struct DummyRegister(u16); },
                parse_quote! { #[xx(SBOP, #mask_missing_bit)] struct DummyRegister(u16); }
            ]);
        }

        for incomplete_mask in incomplete_masks {
            let u16_ident = Ident::new("u16", Span::call_site());
            let result = RegisterDefinition::<u16>::try_parse(&incomplete_mask, u16_ident);

            let error = result.map(|_| ()).unwrap_err();
            expect!(error).to_contain("incomplete field mask");
        }
    }
}
