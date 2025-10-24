use quote::ToTokens;
use syn::Attribute;

use super::RegisterDatasheetAttribute;

#[derive(Clone, Debug)]
enum ReservedRegisterField {
    UnknownSetZeroOrPreserved,
    UnknownSetOneOrPreserved,
    SetZero,
    SetZeroOrPreserved,
    SetOne,
    SetOneOrPreserved
}

#[derive(Clone, Debug)]
struct RegisterFieldAttribute<T: Copy> {
    mask: T,
    reserved: Option<ReservedRegisterField>,
    is_readable: bool,
    is_writable: bool
}

#[derive(Clone, Debug)]
enum RegisterAttribute<T: Copy> {
    Field(RegisterFieldAttribute<T>),
    Datasheet(RegisterDatasheetAttribute)
}

impl<T: Copy> RegisterAttribute<T> {
    pub fn parse(attr: &Attribute) -> Result<RegisterAttribute<T>, String> {
        match attr.path().get_ident() {
            Some(ident) => Self::parse_named(&ident.to_string(), attr),
            _ => Err(format!("Register definition contains an unknown attribute; name={}", attr.path().to_token_stream()))
        }
    }

    fn parse_named(name: &str, attr: &Attribute) -> Result<RegisterAttribute<T>, String> {
        match name {
            "datasheet" => Ok(RegisterAttribute::<T>::Datasheet(RegisterDatasheetAttribute::try_from(attr)?)),
            _ => Err(format!("Register definition contains an unknown attribute; name={}", name))
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use syn::parse_quote;

    use fluent_test::prelude::*;

    use super::*;

    #[test]
    fn parse__called_with_unknown_attribute__expect_err() {
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
            parse_quote! { #[xx::xx(SBZP, 123)] },

            parse_quote! { #[data_sheet("wrong", "snake", 123)] },
            parse_quote! { #[dataSheet("daft", "camel", 123)] },
            parse_quote! { #[Datasheet("rubbish", "pascal", 123)] },
            parse_quote! { #[DATASHEET("incorrect", "allcaps", 123)] }
        ];

        for unknown_attribute in unknown_attributes {
            let result = RegisterAttribute::<u32>::parse(&unknown_attribute);
            expect!(&result).to_be_err();
            expect!(result.unwrap_err().to_string()).to_contain("unknown attribute");
        }
    }

    #[test]
    fn parse__called_with_datasheet_attribute__expect_datasheet_variant() {
        let datasheet_attribute = parse_quote! { #[datasheet("id", "section", 123)] };
        let result = RegisterAttribute::<u32>::parse(&datasheet_attribute).expect("must be ok");
        let is_datasheet = match result { RegisterAttribute::<u32>::Datasheet(_) => true, _ => false };
        expect!(is_datasheet).to_be_true();
    }

    // TODO: mask cannot be zero
}
