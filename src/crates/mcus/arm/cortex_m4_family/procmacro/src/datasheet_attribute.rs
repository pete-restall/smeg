use syn::{Attribute, LitInt, LitStr, Token};
use syn::parse::{Parse, ParseStream};

#[derive(Clone, Debug)]
pub struct RegisterDatasheetAttribute {
    document_id: String,
    section_name: String,
    page_number: usize
}

impl TryFrom<&Attribute> for RegisterDatasheetAttribute {
    type Error = String;

    fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
        if
            attr.path().is_ident("datasheet") &&
            let syn::Meta::List(args) = &attr.meta &&
            let Ok(parsed) = args.parse_args_with(RegisterDatasheetTokens::parse) &&
            let Ok(page_number) = parsed.page_number.base10_parse() &&
            parsed.no_extra_tokens {

            Ok(Self {
                document_id: parsed.document_id.value(),
                section_name: parsed.section_name.value(),
                page_number
            })
        } else {
            Err("Register's datasheet reference is malformed; expected #[datasheet(\"document id\", \"section name\", 123 /* page number */)]".to_string())
        }
    }
}

struct RegisterDatasheetTokens {
    document_id: LitStr,
    _delimiter_1: Token![,],
    section_name: LitStr,
    _delimiter_2: Token![,],
    page_number: LitInt,
    no_extra_tokens: bool
}

impl Parse for RegisterDatasheetTokens {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            document_id: input.parse()?,
            _delimiter_1: input.parse()?,
            section_name: input.parse()?,
            _delimiter_2: input.parse()?,
            page_number: input.parse()?,
            no_extra_tokens: input.is_empty()
        })
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use syn::parse_quote;

    use fluent_test::prelude::*;

    use smeg_testing_host_utils::integers::any_usize;
    use smeg_testing_host_utils::strings::utf8;

    use super::*;

    #[test]
    fn try_from__called_with_unknown_attribute__expect_err() {
        let unknown_attributes: Vec<syn::Attribute> = vec![
            parse_quote! { #[whatever] },
            parse_quote! { #[data_sheet("wrong", "snake", 123)] },
            parse_quote! { #[dataSheet("daft", "camel", 123)] },
            parse_quote! { #[Datasheet("rubbish", "pascal", 123)] },
            parse_quote! { #[DATASHEET("incorrect", "allcaps", 123)] }
        ];

        for unknown_attribute in unknown_attributes {
            let result = RegisterDatasheetAttribute::try_from(&unknown_attribute);
            expect!(&result).to_be_err();
            expect!(result.unwrap_err().to_string()).to_contain("malformed");
        }
    }

    #[test]
    fn try_from__called_with_malformed_datasheet_attribute__expect_err() {
        let malformed_attributes: Vec<syn::Attribute> = vec![
            parse_quote! { #[datasheet] },
            parse_quote! { #[datasheet = "something"] },
            parse_quote! { #[datasheet()] },
            parse_quote! { #[datasheet(NAME)] },
            parse_quote! { #[datasheet(NAME, SECTION)] },
            parse_quote! { #[datasheet(NAME, SECTION, 123)] },
            parse_quote! { #[datasheet("NAME", "SECTION", 123, 456)] },
            parse_quote! { #[datasheet("NAME", "SECTION", -73)] },
            parse_quote! { #[datasheet("NAME")] },
            parse_quote! { #[datasheet("NAME", "SECTION")] }
        ];

        for malformed_attribute in malformed_attributes {
            let result = RegisterDatasheetAttribute::try_from(&malformed_attribute);
            expect!(&result).to_be_err();
            expect!(result.unwrap_err().to_string()).to_contain("malformed");
        }
    }

    #[test]
    fn try_from__called_with_datasheet_attribute__expect_document_id_is_first_argument() {
        try_from__called_with_datasheet_attribute__expect(|actual, expected|
            expect!(actual.document_id).to_equal(expected.document_id));
    }

    fn try_from__called_with_datasheet_attribute__expect<A, F>(assertion: F)
        where F: FnOnce(RegisterDatasheetAttribute, RegisterDatasheetAttribute) -> A {

        let expected = RegisterDatasheetAttribute {
            document_id: utf8::any(),
            section_name: utf8::any(),
            page_number: any_usize()
        };

        let (document_id, section_name, page_number) = (&expected.document_id, &expected.section_name, expected.page_number);
        let datasheet_attribute = parse_quote! { #[datasheet(#document_id, #section_name, #page_number)] };
        let actual = RegisterDatasheetAttribute::try_from(&datasheet_attribute).expect("must be parsed successfully");
        assertion(actual, expected);
    }

    #[test]
    fn try_from__called_with_datasheet_attribute__expect_section_name_is_second_argument() {
        try_from__called_with_datasheet_attribute__expect(|actual, expected|
            expect!(actual.section_name).to_equal(expected.section_name));
    }

    #[test]
    fn try_from__called_with_datasheet_attribute__expect_page_number_is_third_argument() {
        try_from__called_with_datasheet_attribute__expect(|actual, expected|
            expect!(actual.page_number).to_equal(expected.page_number));
    }
}
