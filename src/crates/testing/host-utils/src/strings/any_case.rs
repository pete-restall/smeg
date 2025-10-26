use std::str::Chars;

use crate::booleans::any_bool;

pub trait AnyCase {
    fn any_case(&self) -> String;
}

impl AnyCase for String {
    fn any_case(&self) -> String {
        let dest = String::with_capacity(self.capacity());
        funky_case(dest, self.chars())
    }
}

fn funky_case(mut dest: String, src: Chars<'_>) -> String {
    for ch in src {
        if any_bool() {
            dest.push(ch.to_uppercase().next().unwrap());
        } else {
            dest.push(ch.to_lowercase().next().unwrap());
        }
    }

    dest
}

impl AnyCase for &str {
    fn any_case(&self) -> String {
        let dest = String::with_capacity(self.len());
        funky_case(dest, self.chars())
    }
}
