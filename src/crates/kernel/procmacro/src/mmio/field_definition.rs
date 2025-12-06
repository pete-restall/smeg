use std::fmt::Debug;
use std::ops::BitXor;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Ident;

use super::{RegisterFieldAttribute, RegisterFieldMask, RegisterFieldMaskProperties};

pub struct RegisterFieldDefinition<T> where T: BitXor<Output = T> + Copy + Debug + PartialEq {
    attribute: RegisterFieldAttribute<T>,
    register_type_ident: Ident
}

impl<T> RegisterFieldDefinition<T> where T: BitXor<Output = T> + Copy + Debug + PartialEq {
    pub fn try_parse(attribute: RegisterFieldAttribute<T>, register_type_ident: Ident) -> Result<Self, String> {
        Ok(Self { attribute, register_type_ident })
    }

    pub fn is_readable(&self) -> bool { self.attribute.is_readable() }

    pub fn is_writable(&self) -> bool { self.attribute.is_writable() }
}

impl<T> ToTokens for RegisterFieldDefinition<T>
    where
        T: BitXor<Output = T> + Copy + Debug + PartialEq + ToTokens,
        RegisterFieldMask<T>: RegisterFieldMaskProperties<MaskType = T> {

    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.attribute.reserved().is_some() {
            reserved_field_to_tokens(self, tokens)
        } else {
            nonreserved_field_to_tokens(self, tokens);
        }
    }
}

fn reserved_field_to_tokens<T>(field: &RegisterFieldDefinition<T>, tokens: &mut TokenStream)
    where
        T: BitXor<Output = T> + Copy + Debug + PartialEq + ToTokens,
        RegisterFieldMask<T>: RegisterFieldMaskProperties<MaskType = T> {

    let register_type_ident = &field.register_type_ident;

    let mask_ident = prefixed_suffixed_ident("RESERVED", &field.attribute, "MASK");
    let mask = field.attribute.mask();
    let mask_value = mask.value();

    tokens.append_all(quote! {
        pub const #mask_ident: #register_type_ident = #mask_value;
    });
}

fn prefixed_suffixed_ident<T: BitXor<Output = T> + Copy + Debug + PartialEq>(prefix: &str, attribute: &RegisterFieldAttribute<T>, suffix: &str) -> Ident {
    Ident::new(&format!("{}_{}_{}", prefix, attribute.name_uppercase(), suffix), Span::call_site())
}

fn nonreserved_field_to_tokens<T>(field: &RegisterFieldDefinition<T>, tokens: &mut TokenStream)
    where
        T: BitXor<Output = T> + Copy + Debug + PartialEq + ToTokens,
        RegisterFieldMask<T>: RegisterFieldMaskProperties<MaskType = T> {

    let register_type_ident = &field.register_type_ident;

    let mask_ident = suffixed_ident(&field.attribute, "MASK");
    let mask = field.attribute.mask();
    let mask_value = mask.value();

    let msb_ident = suffixed_ident(&field.attribute, "MSB");
    let mask_msb = quote_optional(mask.most_significant_bit());

    let lsb_ident = suffixed_ident(&field.attribute, "LSB");
    let mask_lsb = quote_optional(mask.least_significant_bit());

    let width_ident = suffixed_ident(&field.attribute, "WIDTH");
    let mask_width = mask.width_bits();

    tokens.append_all(quote! {
        pub const #mask_ident: #register_type_ident = #mask_value;
        pub const #msb_ident: Option<usize> = #mask_msb;
        pub const #lsb_ident: Option<usize> = #mask_lsb;
        pub const #width_ident: usize = #mask_width;
    });
}

fn suffixed_ident<T: BitXor<Output = T> + Copy + Debug + PartialEq>(attribute: &RegisterFieldAttribute<T>, suffix: &str) -> Ident {
    Ident::new(&format!("{}_{}", attribute.name_uppercase(), suffix), Span::call_site())
}

fn quote_optional<T: ToTokens>(maybe: Option<T>) -> TokenStream {
    maybe.map(|value| quote! { Some(#value) }).unwrap_or(quote! { None })
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use proc_macro2::Span;
    use syn::{Attribute, parse_quote};

    use fluent_test::prelude::*;

    use smeg_testing_host_utils::seq::any_item_from;
    use smeg_testing_host_utils::strings::ascii::any_rust_identifier;

    use super::*;

    #[test]
    fn register_type_ident__get_try_after_parse__expect_same_value_passed_to_constructor() {
        let attribute = dummy_field_attribute();
        let register_type_ident = dummy_register_type_ident();
        let field = RegisterFieldDefinition::try_parse(attribute, register_type_ident.clone()).expect("must be ok");
        expect!(field.register_type_ident).to_equal(register_type_ident);
    }

    fn dummy_register_type_ident() -> Ident {
        let any_rust_identifier = any_rust_identifier();
        Ident::new(&any_rust_identifier, Span::call_site())
    }

    fn dummy_field_attribute() -> RegisterFieldAttribute<usize> {
        let attribute: Attribute = parse_quote! { #[ro(X, 1)] };
        RegisterFieldAttribute::try_from(&attribute).expect("must be valid attribute")
    }

    #[test]
    fn attribute__get_after_try_parse__expect_same_value_passed_to_constructor() {
        let attribute = dummy_field_attribute();
        let register_type_ident = dummy_register_type_ident();
        let field = RegisterFieldDefinition::try_parse(attribute.clone(), register_type_ident).expect("must be ok");
        expect!(field.attribute).to_equal(attribute);
    }

    #[test]
    fn is_readable__called_when_attribute_is_unreadable_field__expect_false_is_returned() {
        let attribute = stub_unreadable_field_attribute();
        let register_type_ident = dummy_register_type_ident();
        let field = RegisterFieldDefinition::try_parse(attribute, register_type_ident).expect("must be ok");
        expect!(field.is_readable()).to_be_false();
    }

    fn stub_unreadable_field_attribute() -> RegisterFieldAttribute<usize> {
        let unreadable_attributes = [
            parse_quote! { #[wo(X, 1)] },
            parse_quote! { #[xx(WI, 1)] }
        ];

        let attribute = any_item_from(&unreadable_attributes);
        RegisterFieldAttribute::try_from(attribute).expect("must be valid attribute")
    }

    #[test]
    fn is_readable__called_when_attribute_is_readable_field__expect_true_is_returned() {
        let attribute = stub_readable_field_attribute();
        let register_type_ident = dummy_register_type_ident();
        let field = RegisterFieldDefinition::try_parse(attribute, register_type_ident).expect("must be ok");
        expect!(field.is_readable()).to_be_true();
    }

    fn stub_readable_field_attribute() -> RegisterFieldAttribute<usize> {
        let readable_attributes = [
            parse_quote! { #[ro(X, 1)] },
            parse_quote! { #[rw(X, 1)] }
        ];

        let attribute = any_item_from(&readable_attributes);
        RegisterFieldAttribute::try_from(attribute).expect("must be valid attribute")
    }

    #[test]
    fn is_writable__called_when_attribute_is_unwritable_field__expect_false_is_returned() {
        let attribute = stub_unwritable_field_attribute();
        let register_type_ident = dummy_register_type_ident();
        let field = RegisterFieldDefinition::try_parse(attribute, register_type_ident).expect("must be ok");
        expect!(field.is_writable()).to_be_false();
    }

    fn stub_unwritable_field_attribute() -> RegisterFieldAttribute<usize> {
        let unwritable_attributes = [
            parse_quote! { #[ro(X, 1)] },
            parse_quote! { #[xx(WI, 1)] }
        ];

        let attribute = any_item_from(&unwritable_attributes);
        RegisterFieldAttribute::try_from(attribute).expect("must be valid attribute")
    }

    #[test]
    fn is_writable__called_when_attribute_is_writable_field__expect_true_is_returned() {
        let attribute = stub_writable_field_attribute();
        let register_type_ident = dummy_register_type_ident();
        let field = RegisterFieldDefinition::try_parse(attribute, register_type_ident).expect("must be ok");
        expect!(field.is_writable()).to_be_true();
    }

    fn stub_writable_field_attribute() -> RegisterFieldAttribute<usize> {
        let writable_attributes = [
            parse_quote! { #[wo(X, 1)] },
            parse_quote! { #[rw(X, 1)] }
        ];

        let attribute = any_item_from(&writable_attributes);
        RegisterFieldAttribute::try_from(attribute).expect("must be valid attribute")
    }
}
