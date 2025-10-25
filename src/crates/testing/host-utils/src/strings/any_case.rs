use std::str::Chars;

use rand::RngCore;

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
    let mut rng = rand::rng();
    for ch in src {
        if (rng.next_u32() & 1) != 0 {
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
