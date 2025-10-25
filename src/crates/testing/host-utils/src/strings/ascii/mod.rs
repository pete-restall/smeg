mod rust_identifier_chars;
pub use rust_identifier_chars::*;

pub fn any_rust_identifier() -> String {
    let initial = super::any_string_of(RustIdentifierInitialChars, 1..=1);
    let min_chars = if initial == "_" { 1 } else { 0 };
    initial + &super::any_string_of(RustIdentifierChars, min_chars..=31)
}
