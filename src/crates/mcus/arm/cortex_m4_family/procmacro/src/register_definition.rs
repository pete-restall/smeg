use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

use super::{RegisterAttribute, RegisterDatasheetAttribute, RegisterFieldAttribute, Single};

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

    _type: PhantomData<T>
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
            PartialEq,
        T::Err: Display {

    pub fn generate(derive: &DeriveInput, type_ident: &Ident) -> Result<TokenStream, String> {
        let attributes = derive.attrs.iter().map(RegisterAttribute::<T>::parse).collect::<Result<Vec<_>, _>>()?;

        let datasheet = Self::parse_datasheet_from(&attributes)?;
        let fields = Self::parse_fields_from(&attributes)?;

        let all_bits_set = !T::default();
        let mask = Self::mask_for(fields.into_values())?;
        if mask != all_bits_set {
            return Err("Register definition has incomplete field mask; every bit needs to be defined".to_string())
        }

        //iterate fields below and build up consts, etc.
        //figure out if the register is readable (any ro|rw) and writable (any wo|wr)

        let (visibility, register_ident) = (&derive.vis, &derive.ident);
        Ok(quote! {
            #[repr(transparent)]
            #visibility struct #register_ident(#type_ident);

            impl #register_ident {
                // also want some other const booleans - IS_READABLE, IS_WRITABLE, IS_READONLY, IS_WRITEONLY - but put these on a trait
                pub const FIELD_1_MASK: #type_ident = 1 << 31;
                pub const FIELD_1_MSB: #type_ident = 31;
                pub const FIELD_1_LSB: #type_ident = 31;
                pub const FIELD_1_WIDTH: #type_ident = 1;

                pub const IS_READABLE: bool = true; // TODO
                pub const IS_WRITABLE: bool = false; // TODO

                pub const IS_READONLY: bool = Self::IS_READABLE && !Self::IS_WRITABLE;
                pub const IS_WRITEONLY: bool = !Self::IS_READABLE && Self::IS_WRITABLE;
            }

    //        impl<'mem> #accessor_name<'mem> {
                // TODO...can't use generics from 'input'...
    //    pub fn implementer(&self) -> usize { unsafe { self.accessor.mmio_read_masked_shifted_right::<{Cpuid::IMPLEMENTER_MASK}>() } }
    //     pub fn implementer_raw(&self) -> usize { unsafe { self.accessor.mmio_read_masked::<{Cpuid::IMPLEMENTER_MASK}>() } }

            // TODO: #visibility type {#register_name}Cell<M> = {Readonly|Writeonly|ReadWrite}Cell<M, #register_name>;
        })
    }

    fn parse_datasheet_from(attributes: &[RegisterAttribute<T>]) -> Result<Option<&RegisterDatasheetAttribute>, &str> {
        attributes
            .iter()
            .filter_map(|attr| match attr { RegisterAttribute::Datasheet(datasheet) => Some(datasheet), _ => None })
            .single_or_none()
            .map_err(|_| "Register definition cannot have multiple datasheet attributes specified")
    }

    fn parse_fields_from(attributes: &[RegisterAttribute<T>]) -> Result<HashMap<&str, &RegisterFieldAttribute<T>>, &str> {
        let mut fields: HashMap<&str, &RegisterFieldAttribute<T>> = HashMap::default();
        let are_all_fields_unique = attributes
            .iter()
            .filter_map(|attr| match attr { RegisterAttribute::Field(field) => Some(field), _ => None })
            .filter_map(|field| fields.insert(field.name_uppercase(), field))
            .last()
            .is_none();

        if are_all_fields_unique {
            Ok(fields)
        } else {
            Err("Register definition has duplicate field names")
        }
    }

    fn mask_for<'a, I: Iterator<Item = &'a RegisterFieldAttribute<T>>>(mut fields: I) -> Result<T, &'a str>
        where T: 'a {

        let zero = T::default();
        fields.try_fold(zero, |mask, field| if (mask & field.mask()) == zero {
            Ok(mask | field.mask())
        } else {
            Err("Register definition has overlapping field masks")
        })
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
    fn generate__called_when_multiple_datasheets_given__expect_err() {
        let too_many_datasheets: DeriveInput = parse_quote! {
            #[datasheet("somewhere", "section", 123)]
            #[datasheet("elsewhere", "another", 456)]
            #[ro(FIELD_1, 0xff)]
            struct DummyRegister(u8);
        };

        let result = RegisterDefinition::<u8>::generate(
            &too_many_datasheets,
            &Ident::new("u8", Span::call_site()));

        expect!(result.unwrap_err()).to_contain("multiple datasheet");
    }

    #[test]
    fn generate__called_when_field_names_are_not_case_insentitively_unique__expect_err() {
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
            let result = RegisterDefinition::<usize>::generate(
                &malformed_attribute,
                &Ident::new("usize", Span::call_site()));

            expect!(result.unwrap_err()).to_contain("duplicate field");
        }
    }

    #[test]
    fn generate__called_when_field_have_overlapping_masks__expect_err() {
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
            let result = RegisterDefinition::<u32>::generate(
                &malformed_attribute,
                &Ident::new("u32", Span::call_site()));

            expect!(result.unwrap_err()).to_contain("overlapping field masks");
        }
    }

    #[test]
    fn generate__called_when_field_masks_are_zero__expect_err() {
        let no_field_masks: DeriveInput = parse_quote! { struct DummyRegister(u32); };
        let result = RegisterDefinition::<u32>::generate(
            &no_field_masks,
            &Ident::new("u32", Span::call_site()));

        expect!(result.unwrap_err()).to_contain("incomplete field mask");
    }

    #[test]
    fn generate__called_when_field_masks_do_not_cover_every_bit__expect_err() {
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
            let result = RegisterDefinition::<u16>::generate(
                &incomplete_mask,
                &Ident::new("u16", Span::call_site()));

            expect!(result.unwrap_err()).to_contain("incomplete field mask");
        }
    }
}
