use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::ops::BitXor;
use std::str::FromStr;

use syn::{Attribute, Ident, LitInt, Token};
use syn::parse::{Parse, ParseStream};

use super::RegisterFieldMask;

#[derive(Clone, Debug, PartialEq)]
pub struct RegisterFieldAttribute<T: BitXor<Output = T> + Copy + Debug + PartialEq> {
    name_uppercase: String,
    name_lowercase: String,
    mask: RegisterFieldMask<T>,
    reserved: Option<ReservedRegisterField>,
    is_readable: bool,
    is_writable: bool
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ReservedRegisterField {
    UnknownShouldBeZero,
    UnknownShouldBeZeroOrPreserved,
    UnknownShouldBeOne,
    UnknownShouldBeOneOrPreserved,
    UnknownShouldBePreserved,
    ShouldBeZero,
    ShouldBeZeroOrPreserved,
    ShouldBeOne,
    ShouldBeOneOrPreserved,
    ShouldBePreserved,
    WriteIgnored
}

impl<T: BitXor<Output = T> + Copy + Debug + PartialEq> RegisterFieldAttribute<T> {
    pub fn name_uppercase(&self) -> &str { &self.name_uppercase }

    pub fn name_lowercase(&self) -> &str { &self.name_lowercase }

    pub fn mask(&self) -> RegisterFieldMask<T> { self.mask }

    pub fn reserved(&self) -> Option<ReservedRegisterField> { self.reserved }

    pub fn is_readable(&self) -> bool { self.is_readable }

    pub fn is_writable(&self) -> bool { self.is_writable }
}

impl<T> TryFrom<&Attribute> for RegisterFieldAttribute<T>
    where
        T: BitXor<Output = T> + Copy + Debug + Display + FromStr + PartialEq + Into<RegisterFieldMask<T>>,
        T::Err: Display {

    type Error = String;

    fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
        if
            let Some(ident) = attr.path().get_ident() &&
            (ident == "ro" || ident == "wo" || ident == "rw" || ident == "xx") &&
            let syn::Meta::List(args) = &attr.meta &&
            let Ok(parsed) = args.parse_args_with(RegisterFieldTokens::parse) &&
            let Ok(mask) = parsed.mask.base10_parse::<T>() &&
            parsed.no_extra_tokens {

            if is_not_zero(mask) {
                let field_name = parsed.field_name.to_string();
                let name_uppercase = field_name.to_uppercase();
                let name_lowercase = field_name.to_lowercase();
                Ok(Self {
                    reserved: match (ident == "xx", name_uppercase.as_str()) {
                        (true, "UNK_SBZ") => Ok(Some(ReservedRegisterField::UnknownShouldBeZero)),
                        (true, "UNK_SBZP") => Ok(Some(ReservedRegisterField::UnknownShouldBeZeroOrPreserved)),
                        (true, "UNK_SBO") => Ok(Some(ReservedRegisterField::UnknownShouldBeOne)),
                        (true, "UNK_SBOP") => Ok(Some(ReservedRegisterField::UnknownShouldBeOneOrPreserved)),
                        (true, "UNK_SBP") => Ok(Some(ReservedRegisterField::UnknownShouldBePreserved)),
                        (true, "SBZ") => Ok(Some(ReservedRegisterField::ShouldBeZero)),
                        (true, "SBZP") => Ok(Some(ReservedRegisterField::ShouldBeZeroOrPreserved)),
                        (true, "SBO") => Ok(Some(ReservedRegisterField::ShouldBeOne)),
                        (true, "SBOP") => Ok(Some(ReservedRegisterField::ShouldBeOneOrPreserved)),
                        (true, "SBP") => Ok(Some(ReservedRegisterField::ShouldBePreserved)),
                        (true, "WI") => Ok(Some(ReservedRegisterField::WriteIgnored)),
                        (true, _) => Err(format!("Register's field definition is malformed due to an unknown reserved field name; name={}", field_name)),
                        _ => Ok(None)
                    }?,
                    name_uppercase,
                    name_lowercase,
                    mask: mask.into(),
                    is_readable: ident == "ro" || ident == "rw",
                    is_writable: ident == "wo" || ident == "rw"
                })
            } else {
                Err("Register's field definition is malformed; mask must not be zero".to_string())
            }
        } else {
            Err("Register's field definition is malformed; expected #[__(NAME, 0b01...01 /* mask */)], where __ is one of ro|wo|rw|xx".to_string())
        }
    }
}

#[allow(clippy::eq_op)]
fn is_not_zero<T: BitXor<Output = T> + Copy + PartialEq>(x: T) -> bool {
    (x ^ x) != x
}

struct RegisterFieldTokens {
    field_name: Ident,
    _delimiter_1: Token![,],
    mask: LitInt,
    no_extra_tokens: bool
}

impl Parse for RegisterFieldTokens {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            field_name: input.parse()?,
            _delimiter_1: input.parse()?,
            mask: input.parse()?,
            no_extra_tokens: input.is_empty()
        })
    }
}

pub(crate) fn default_reserved_register_fields<T>() -> [RegisterFieldAttribute<T>; 11]
    where T: BitXor<Output = T> + Copy + Debug + Default + Into<RegisterFieldMask<T>> + PartialEq {

    [
        default_reserved_register_field(ReservedRegisterField::UnknownShouldBeZero, "UNK_SBZ"),
        default_reserved_register_field(ReservedRegisterField::UnknownShouldBeZeroOrPreserved, "UNK_SBZP"),
        default_reserved_register_field(ReservedRegisterField::UnknownShouldBeOne, "UNK_SBO"),
        default_reserved_register_field(ReservedRegisterField::UnknownShouldBeOneOrPreserved, "UNK_SBOP"),
        default_reserved_register_field(ReservedRegisterField::UnknownShouldBePreserved, "UNK_SBP"),
        default_reserved_register_field(ReservedRegisterField::ShouldBeZero, "SBZ"),
        default_reserved_register_field(ReservedRegisterField::ShouldBeZeroOrPreserved, "SBZP"),
        default_reserved_register_field(ReservedRegisterField::ShouldBeOne, "SBO"),
        default_reserved_register_field(ReservedRegisterField::ShouldBeOneOrPreserved, "SBOP"),
        default_reserved_register_field(ReservedRegisterField::ShouldBePreserved, "SBP"),
        default_reserved_register_field(ReservedRegisterField::WriteIgnored, "WI")
    ]
}

fn default_reserved_register_field<T>(reserved: ReservedRegisterField, name: &str) -> RegisterFieldAttribute<T>
    where T: BitXor<Output = T> + Copy + Debug + Default + Into<RegisterFieldMask<T>> + PartialEq {

    RegisterFieldAttribute {
        name_uppercase: name.to_uppercase(),
        name_lowercase: name.to_lowercase(),
        mask: (T::default() ^ T::default()).into(),
        reserved: Some(reserved),
        is_readable: false,
        is_writable: false
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use proc_macro2::Span;
    use quote::ToTokens;
    use syn::parse_quote;

    use fluent_test::prelude::*;

    use smeg_testing_host_utils::booleans::any_bool;
    use smeg_testing_host_utils::integers::{any_usize, any_usize_except};
    use smeg_testing_host_utils::strings::{ascii, utf8, AnyCase};

    use super::super::RegisterFieldMaskProperties;
    use super::*;

    #[test]
    fn name_uppercase__called__expect_slice_of_name_uppercase_field() {
        let attribute = stub_attribute_struct();
        expect!(attribute.name_uppercase()).to_equal(&attribute.name_uppercase);
    }

    fn stub_attribute_struct() -> RegisterFieldAttribute<usize> {
        RegisterFieldAttribute {
            name_uppercase: utf8::any(),
            name_lowercase: utf8::any(),
            mask: any_usize().into(),
            reserved: None,
            is_readable: any_bool(),
            is_writable: any_bool()
        }
    }

    #[test]
    fn name_lowercase__called__expect_slice_of_name_lowercase_field() {
        let attribute = stub_attribute_struct();
        expect!(attribute.name_lowercase()).to_equal(&attribute.name_lowercase);
    }

    #[test]
    fn mask__called__expect_mask_field_is_returned() {
        let attribute = stub_attribute_struct();
        expect!(attribute.mask()).to_equal(attribute.mask);
    }

    #[test]
    fn reserved__called_when_field_is_none__expect_none_is_returned() {
        let attribute = stub_attribute_struct_for_reserved(None);
        expect!(attribute.reserved().is_none()).to_be_true();
    }

    fn stub_attribute_struct_for_reserved(reserved: Option<ReservedRegisterField>) -> RegisterFieldAttribute<usize> {
        let mut attribute = stub_attribute_struct();
        attribute.reserved = reserved;
        attribute
    }

    #[test]
    fn reserved__called_when_field_is_some__expect_same_value_is_returned() {
        for reserved in KNOWN_RESERVED_FIELD_TYPES {
            let attribute = stub_attribute_struct_for_reserved(Some(reserved));
            expect!(attribute.reserved().unwrap()).to_equal(reserved);
        }
    }

    static KNOWN_RESERVED_FIELD_TYPES: [ReservedRegisterField; 11] = [
        ReservedRegisterField::UnknownShouldBeZero,
        ReservedRegisterField::UnknownShouldBeZeroOrPreserved,
        ReservedRegisterField::UnknownShouldBeOne,
        ReservedRegisterField::UnknownShouldBeOneOrPreserved,
        ReservedRegisterField::UnknownShouldBePreserved,
        ReservedRegisterField::ShouldBeZero,
        ReservedRegisterField::ShouldBeZeroOrPreserved,
        ReservedRegisterField::ShouldBeOne,
        ReservedRegisterField::ShouldBeOneOrPreserved,
        ReservedRegisterField::ShouldBePreserved,
        ReservedRegisterField::WriteIgnored];

    #[test]
    fn is_readable__called__expect_same_value_as_field_is_returned() {
        for is_readable in [true, false] {
            let mut attribute = stub_attribute_struct();
            attribute.is_readable = is_readable;
            {
                let attribute = attribute;
                expect!(attribute.is_readable()).to_equal(is_readable);
            }
        }
    }

    #[test]
    fn is_writable__called__expect_same_value_as_field_is_returned() {
        for is_writable in [true, false] {
            let mut attribute = stub_attribute_struct();
            attribute.is_writable = is_writable;
            {
                let attribute = attribute;
                expect!(attribute.is_writable()).to_equal(is_writable);
            }
        }
    }

    #[test]
    fn try_from__called_with_unknown_attribute__expect_err() {
        let unknown_attributes: Vec<syn::Attribute> = vec![
            parse_quote! { #[whatever] },
            parse_quote! { #[Ro(FIELD_1, 123)] },
            parse_quote! { #[RO(FIELD_1, 123)] },
            parse_quote! { #[_ro(FIELD_1, 123)] },
            parse_quote! { #[::ro(FIELD_1, 123)] },
            parse_quote! { #[ro::ro(FIELD_1, 123)] },
            parse_quote! { #[readonly(FIELD_1, 123)] },

            parse_quote! { #[Wo(FIELD_1, 123)] },
            parse_quote! { #[WO(FIELD_1, 123)] },
            parse_quote! { #[_wo(FIELD_1, 123)] },
            parse_quote! { #[::wo(FIELD_1, 123)] },
            parse_quote! { #[wo::wo(FIELD_1, 123)] },
            parse_quote! { #[writeonly(FIELD_1, 123)] },

            parse_quote! { #[Rw(FIELD_1, 123)] },
            parse_quote! { #[RW(FIELD_1, 123)] },
            parse_quote! { #[_rw(FIELD_1, 123)] },
            parse_quote! { #[::rw(FIELD_1, 123)] },
            parse_quote! { #[rw::rw(FIELD_1, 123)] },
            parse_quote! { #[readwrite(FIELD_1, 123)] },

            parse_quote! { #[Xx(SBZP, 123)] },
            parse_quote! { #[XX(SBZP, 123)] },
            parse_quote! { #[_xx(SBZP, 123)] },
            parse_quote! { #[xxx(SBZP, 123)] },
            parse_quote! { #[::xx(SBZP, 123)] },
            parse_quote! { #[xx::xx(SBZP, 123)] }
        ];

        for unknown_attribute in unknown_attributes {
            let result = RegisterFieldAttribute::<usize>::try_from(&unknown_attribute);
            expect!(&result).to_be_err();
            expect!(result.unwrap_err().to_string()).to_contain("malformed");
        }
    }

    #[test]
    fn try_from__called_with_malformed_ro_field_attribute__expect_err() {
        try_from__called_with_malformed_field_attribute__expect_err::<u8>(vec![
            parse_quote! { #[ro] },
            parse_quote! { #[ro = WRONG_SYNTAX] },
            parse_quote! { #[ro()] },
            parse_quote! { #[ro(NO_MASK)] },
            parse_quote! { #[ro(STRING_NOT_INT, "123")] },
            parse_quote! { #[ro(FLOAT_NOT_ALLOWED, 123.456)] },
            parse_quote! { #[ro("STRING_NOT_IDENT", 123)] },
            parse_quote! { #[ro(NEGATIVE_FOR_U8, -123)] },
            parse_quote! { #[ro(TOO_BIG_FOR_U8, 456)] },
            parse_quote! { #[ro(ZERO_MASK_IS_POINTLESS, 0)] }
        ]);
    }

    fn try_from__called_with_malformed_field_attribute__expect_err<T>(malformed_attributes: Vec<syn::Attribute>)
        where
            T: BitXor<Output = T> + Copy + Debug + Display + FromStr + PartialEq + Into<RegisterFieldMask<T>>,
            RegisterFieldMask<T>: RegisterFieldMaskProperties<MaskType = T>,
            T::Err: Display {

        for malformed_attribute in malformed_attributes {
            let result = RegisterFieldAttribute::<T>::try_from(&malformed_attribute);
            expect!(&result).to_be_err();
            expect!(result.unwrap_err().to_string()).to_contain("malformed");
        }
    }

    #[test]
    fn try_from__called_with_malformed_wo_field_attribute__expect_err() {
        try_from__called_with_malformed_field_attribute__expect_err::<i8>(vec![
            parse_quote! { #[wo] },
            parse_quote! { #[wo = WRONG_SYNTAX] },
            parse_quote! { #[wo()] },
            parse_quote! { #[wo(NO_MASK)] },
            parse_quote! { #[wo(STRING_NOT_INT, "123")] },
            parse_quote! { #[wo(FLOAT_NOT_ALLOWED, 123.456)] },
            parse_quote! { #[wo("STRING_NOT_IDENT", 123)] },
            parse_quote! { #[wo(TOO_SMALL_FOR_I8, -129)] },
            parse_quote! { #[wo(TOO_BIG_FOR_I8, 128)] },
            parse_quote! { #[wo(ZERO_MASK_IS_POINTLESS, 0)] }
        ]);
    }

    #[test]
    fn try_from__called_with_malformed_rw_field_attribute__expect_err() {
        try_from__called_with_malformed_field_attribute__expect_err::<usize>(vec![
            parse_quote! { #[rw] },
            parse_quote! { #[rw = WRONG_SYNTAX] },
            parse_quote! { #[rw()] },
            parse_quote! { #[rw(NO_MASK)] },
            parse_quote! { #[rw(STRING_NOT_INT, "123")] },
            parse_quote! { #[rw(FLOAT_NOT_ALLOWED, 123.456)] },
            parse_quote! { #[rw("STRING_NOT_IDENT", 123)] },
            parse_quote! { #[rw(ZERO_MASK_IS_POINTLESS, 0)] }
        ]);
    }

    #[test]
    fn try_from__called_with_malformed_xx_field_attribute__expect_err() {
        try_from__called_with_malformed_field_attribute__expect_err::<isize>(vec![
            parse_quote! { #[xx] },
            parse_quote! { #[xx = WRONG_SYNTAX] },
            parse_quote! { #[xx()] },
            parse_quote! { #[xx(SBZP /* no mask */)] },
            parse_quote! { #[xx(SBOP, "123" /* string not int */)] },
            parse_quote! { #[xx(UNK_SBP, 123.456 /* float not allowed */)] },
            parse_quote! { #[xx("UNK_SBZ", 123 /* string not ident */)] },
            parse_quote! { #[xx(UNK_SBZP, 0 /* zero mask is pointless */)] },
            parse_quote! { #[xx(UNKNOWN_TYPE_OF_RESERVED_FIELD, 123)] },
            parse_quote! { #[xx(ANOTHER_UNKNOWN_FIELD_NAME, 1)] }
        ]);
    }

    #[test]
    fn try_from__called_with_ro_attribute__expect_name_uppercase_is_uppercased_first_argument() {
        try_from__called_with_ro_attribute__expect(|actual, expected|
            expect!(actual.name_uppercase).to_equal(expected.name_uppercase));
    }

    fn try_from__called_with_ro_attribute__expect<A, F>(assertion: F)
        where F: FnOnce(RegisterFieldAttribute<usize>, RegisterFieldAttribute<usize>) -> A {

        let field_name = ascii::any_rust_identifier();
        let expected = RegisterFieldAttribute {
            name_uppercase: field_name.to_uppercase(),
            name_lowercase: field_name.to_lowercase(),
            mask: any_usize_except(0).into(),
            reserved: None,
            is_readable: true,
            is_writable: false
        };

        try_from__called_with_attribute__expect("ro", expected, assertion);
    }

    fn try_from__called_with_attribute__expect<A, F, T>(attribute_name: &str, expected: RegisterFieldAttribute<T>, assertion: F)
        where
            F: FnOnce(RegisterFieldAttribute<T>, RegisterFieldAttribute<T>) -> A,
            T: BitXor<Output = T> + Copy + Debug + Display + FromStr + PartialEq + ToTokens + Into<RegisterFieldMask<T>>,
            RegisterFieldMask<T>: RegisterFieldMaskProperties<MaskType = T>,
            RegisterFieldAttribute<T>: for<'a> TryFrom<&'a Attribute, Error = String> {

        let attribute_ident = Ident::new(attribute_name, Span::call_site());
        let (field_name, mask) = (Ident::new(&expected.name_lowercase.any_case(), Span::call_site()), expected.mask.value());
        let attribute = parse_quote! { #[#attribute_ident(#field_name, #mask)] };
        let actual = RegisterFieldAttribute::<T>::try_from(&attribute).expect("must be parsed successfully");
        assertion(actual, expected);
    }

    #[test]
    fn try_from__called_with_ro_attribute__expect_name_lowercase_is_lowercased_first_argument() {
        try_from__called_with_ro_attribute__expect(|actual, expected|
            expect!(actual.name_lowercase).to_equal(expected.name_lowercase));
    }

    #[test]
    fn try_from__called_with_ro_attribute__expect_mask_is_second_argument() {
        try_from__called_with_ro_attribute__expect(|actual, expected|
            expect!(actual.mask).to_equal(expected.mask));
    }

    #[test]
    fn try_from__called_with_ro_attribute__expect_reserved_is_none() {
        try_from__called_with_ro_attribute__expect(|actual, _| expect!(actual.reserved.is_none()).to_be_true());
    }

    #[test]
    fn try_from__called_with_ro_attribute__expect_readable_flag_is_true() {
        try_from__called_with_ro_attribute__expect(|actual, _| expect!(actual.is_readable).to_be_true());
    }

    #[test]
    fn try_from__called_with_ro_attribute__expect_writable_flag_is_false() {
        try_from__called_with_ro_attribute__expect(|actual, _| expect!(actual.is_writable).to_be_false());
    }

    #[test]
    fn try_from__called_with_wo_attribute__expect_name_uppercase_is_uppercased_first_argument() {
        try_from__called_with_wo_attribute__expect(|actual, expected|
            expect!(actual.name_uppercase).to_equal(expected.name_uppercase));
    }

    fn try_from__called_with_wo_attribute__expect<A, F>(assertion: F)
        where F: FnOnce(RegisterFieldAttribute<usize>, RegisterFieldAttribute<usize>) -> A {

        let field_name = ascii::any_rust_identifier();
        let expected = RegisterFieldAttribute {
            name_uppercase: field_name.to_uppercase(),
            name_lowercase: field_name.to_lowercase(),
            mask: any_usize_except(0).into(),
            reserved: None,
            is_readable: false,
            is_writable: true
        };

        try_from__called_with_attribute__expect("wo", expected, assertion);
    }

    #[test]
    fn try_from__called_with_wo_attribute__expect_name_lowercase_is_lowercased_first_argument() {
        try_from__called_with_wo_attribute__expect(|actual, expected|
            expect!(actual.name_lowercase).to_equal(expected.name_lowercase));
    }

    #[test]
    fn try_from__called_with_wo_attribute__expect_mask_is_second_argument() {
        try_from__called_with_wo_attribute__expect(|actual, expected|
            expect!(actual.mask).to_equal(expected.mask));
    }

    #[test]
    fn try_from__called_with_wo_attribute__expect_reserved_is_none() {
        try_from__called_with_wo_attribute__expect(|actual, _| expect!(actual.reserved.is_none()).to_be_true());
    }

    #[test]
    fn try_from__called_with_wo_attribute__expect_readable_flag_is_false() {
        try_from__called_with_wo_attribute__expect(|actual, _| expect!(actual.is_readable).to_be_false());
    }

    #[test]
    fn try_from__called_with_wo_attribute__expect_writable_flag_is_true() {
        try_from__called_with_wo_attribute__expect(|actual, _| expect!(actual.is_writable).to_be_true());
    }

    #[test]
    fn try_from__called_with_rw_attribute__expect_name_uppercase_is_uppercased_first_argument() {
        try_from__called_with_rw_attribute__expect(|actual, expected|
            expect!(actual.name_uppercase).to_equal(expected.name_uppercase));
    }

    fn try_from__called_with_rw_attribute__expect<A, F>(assertion: F)
        where F: FnOnce(RegisterFieldAttribute<usize>, RegisterFieldAttribute<usize>) -> A {

        let field_name = ascii::any_rust_identifier();
        let expected = RegisterFieldAttribute {
            name_uppercase: field_name.to_uppercase(),
            name_lowercase: field_name.to_lowercase(),
            mask: any_usize_except(0).into(),
            reserved: None,
            is_readable: true,
            is_writable: true
        };

        try_from__called_with_attribute__expect("rw", expected, assertion);
    }

    #[test]
    fn try_from__called_with_rw_attribute__expect_name_lowercase_is_lowercased_first_argument() {
        try_from__called_with_rw_attribute__expect(|actual, expected|
            expect!(actual.name_lowercase).to_equal(expected.name_lowercase));
    }

    #[test]
    fn try_from__called_with_rw_attribute__expect_mask_is_second_argument() {
        try_from__called_with_rw_attribute__expect(|actual, expected|
            expect!(actual.mask).to_equal(expected.mask));
    }

    #[test]
    fn try_from__called_with_rw_attribute__expect_reserved_is_none() {
        try_from__called_with_rw_attribute__expect(|actual, _| expect!(actual.reserved.is_none()).to_be_true());
    }

    #[test]
    fn try_from__called_with_rw_attribute__expect_readable_flag_is_true() {
        try_from__called_with_rw_attribute__expect(|actual, _| expect!(actual.is_readable).to_be_true());
    }

    #[test]
    fn try_from__called_with_rw_attribute__expect_writable_flag_is_true() {
        try_from__called_with_rw_attribute__expect(|actual, _| expect!(actual.is_writable).to_be_true());
    }

    static KNOWN_RESERVED_FIELD_NAMES: [&str; 11] = [
        "UNK_SBZ",
        "UNK_SBZP",
        "UNK_SBO",
        "UNK_SBOP",
        "UNK_SBP",
        "SBZ",
        "SBZP",
        "SBO",
        "SBOP",
        "SBP",
        "WI"];

    #[test]
    fn try_from__called_with_xx_attribute_and_known_field_name__expect_name_uppercase_is_uppercased_first_argument() {
        for field_name in KNOWN_RESERVED_FIELD_NAMES {
            let field_name = field_name.any_case();
            try_from__called_with_xx_attribute__expect(
                field_name,
                |actual, expected| expect!(actual.name_uppercase).to_equal(expected.name_uppercase));
        }
    }

    fn try_from__called_with_xx_attribute__expect<A, F>(field_name: String, assertion: F)
        where F: FnOnce(RegisterFieldAttribute<usize>, RegisterFieldAttribute<usize>) -> A {

        let name_uppercase = field_name.to_uppercase();
        let expected = RegisterFieldAttribute {
            reserved: Some(match name_uppercase.as_str() {
                "UNK_SBZ" => ReservedRegisterField::UnknownShouldBeZero,
                "UNK_SBZP" => ReservedRegisterField::UnknownShouldBeZeroOrPreserved,
                "UNK_SBO" => ReservedRegisterField::UnknownShouldBeOne,
                "UNK_SBOP" => ReservedRegisterField::UnknownShouldBeOneOrPreserved,
                "UNK_SBP" => ReservedRegisterField::UnknownShouldBePreserved,
                "SBZ" => ReservedRegisterField::ShouldBeZero,
                "SBZP" => ReservedRegisterField::ShouldBeZeroOrPreserved,
                "SBO" => ReservedRegisterField::ShouldBeOne,
                "SBOP" => ReservedRegisterField::ShouldBeOneOrPreserved,
                "SBP" => ReservedRegisterField::ShouldBePreserved,
                "WI" => ReservedRegisterField::WriteIgnored,
                _ => panic!("Unknown reserved field name passed to stub")
            }),
            name_uppercase,
            name_lowercase: field_name.to_lowercase(),
            mask: any_usize_except(0).into(),
            is_readable: false,
            is_writable: false
        };

        try_from__called_with_attribute__expect("xx", expected, assertion);
    }

    #[test]
    fn try_from__called_with_xx_attribute_and_known_field_name__expect_name_lowercase_is_lowercased_first_argument() {
        for field_name in KNOWN_RESERVED_FIELD_NAMES {
            let field_name = field_name.any_case();
            try_from__called_with_xx_attribute__expect(
                field_name,
                |actual, expected| expect!(actual.name_lowercase).to_equal(expected.name_lowercase));
        }
    }

    #[test]
    fn try_from__called_with_xx_attribute__expect_mask_is_second_argument() {
        for field_name in KNOWN_RESERVED_FIELD_NAMES {
            try_from__called_with_xx_attribute__expect(
                field_name.any_case(),
                |actual, expected| expect!(actual.mask).to_equal(expected.mask));
        }
    }

    #[test]
    fn try_from__called_with_xx_attribute__expect_reserved_is_variant_corresponding_to_field_name() {
        for field_name in KNOWN_RESERVED_FIELD_NAMES {
            try_from__called_with_xx_attribute__expect(
                field_name.any_case(),
                |actual, expected| expect!(actual.reserved.unwrap()).to_equal(expected.reserved.unwrap()));
        }
    }

    #[test]
    fn try_from__called_with_xx_attribute__expect_readable_flag_is_false() {
        for field_name in KNOWN_RESERVED_FIELD_NAMES {
            try_from__called_with_xx_attribute__expect(
                field_name.any_case(),
                |actual, _| expect!(actual.is_readable).to_be_false());
        }
    }

    #[test]
    fn try_from__called_with_xx_attribute__expect_writable_flag_is_false() {
        for field_name in KNOWN_RESERVED_FIELD_NAMES {
            try_from__called_with_xx_attribute__expect(
                field_name.any_case(),
                |actual, _| expect!(actual.is_writable).to_be_false());
        }
    }
}
