use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::BitXor;
use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

use super::{RegisterAttribute, RegisterDatasheetAttribute, RegisterFieldAttribute, Single};

pub struct RegisterDefinitionGenerator<T>
    where
        T: BitXor<Output = T> + Copy + Debug + Display + FromStr + PartialEq,
        T::Err: Display {

    _type: PhantomData<T>
}

impl<T> RegisterDefinitionGenerator<T>
    where
        T: BitXor<Output = T> + Copy + Debug + Display + FromStr + PartialEq,
        T::Err: Display {

    pub fn generate(derive: &DeriveInput, type_ident: &Ident) -> Result<TokenStream, String> {
        let attributes = derive.attrs.iter().map(RegisterAttribute::<T>::parse).collect::<Result<Vec<_>, _>>()?;

        let datasheet = Self::parse_datasheet_from(&attributes)?;
        let fields = Self::parse_fields_from(&attributes)?;

        //test all fields for overlapping bits; else panic
        //test or'd fields to ensure all bits set; else panic
        //group all reserved fields by their type and or them
        //iterate fields below and build up consts, etc.
        //figure out if the register is readable (any ro|rw) and writable (any wo|wr)

        let (visibility, register_ident) = (&derive.vis, &derive.ident);
        Ok(quote! {
            #[repr(transparent)]
            #[derive(Copy, Clone)]
            #visibility struct #register_ident(#type_ident);

            impl #register_ident {
                // also want some other const booleans - IS_READABLE, IS_WRITABLE, IS_READONLY, IS_WRITEONLY - but put these on a trait
                pub const FIELD_1_MASK: #type_ident = 1 << 31;
                pub const FIELD_1_MSB: #type_ident = 31;
                pub const FIELD_1_LSB: #type_ident = 31;
                pub const FIELD_1_WIDTH: #type_ident = 1;
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
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use proc_macro2::Span;
    use syn::parse_quote;

    use fluent_test::prelude::*;

    use smeg_testing_host_utils::integers::any_usize_except_in;
    use smeg_testing_host_utils::strings::AnyCase;
    use smeg_testing_host_utils::strings::ascii::any_rust_identifier;

    use super::*;

    #[test]
    fn generate__called_with_multiple_datasheet_attributes__expect_err() {
        let too_many_datasheets: DeriveInput = parse_quote! {
            #[datasheet("somewhere", "section", 123)]
            #[datasheet("elsewhere", "another", 456)]
            #[ro(FIELD_1, 0xff)]
            struct DummyRegister(u8);
        };

        let result = RegisterDefinitionGenerator::<u8>::generate(
            &too_many_datasheets,
            &Ident::new("u8", Span::call_site()));

        expect!(result.unwrap_err()).to_contain("multiple datasheet");
    }

    #[test]
    fn generate__called_when_attributes_have_duplicate_case_insensitive_field_names__expect_err() {
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
            let result = RegisterDefinitionGenerator::<usize>::generate(
                &malformed_attribute,
                &Ident::new("usize", Span::call_site()));

            expect!(result.unwrap_err()).to_contain("duplicate field");
        }
    }
}
