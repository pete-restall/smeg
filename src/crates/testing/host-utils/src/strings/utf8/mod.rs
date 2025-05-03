mod all_chars;
pub use all_chars::*;

mod whitespace_chars;
pub use whitespace_chars::*;

pub fn any() -> String {
    super::any_string_of(AllChars, 0..=32)
}

pub fn any_nonempty() -> String {
    super::any_string_of(AllChars, 1..=32)
}

pub fn any_whitespace() -> String {
    super::any_string_of(WhitespaceChars, 1..=32)
}
