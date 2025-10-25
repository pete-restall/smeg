use rand::Rng;
use rand::distr::{Distribution, SampleString};
use rand::seq::IndexedRandom;

pub struct RustIdentifierInitialChars;

impl Distribution<char> for RustIdentifierInitialChars {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        static CHARS: &[u8] = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
        *CHARS.choose(rng).unwrap() as char
    }
}

impl SampleString for RustIdentifierInitialChars {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        super::super::sample_append_string(RustIdentifierInitialChars, rng, string, len);
    }
}

pub struct RustIdentifierChars;

impl Distribution<char> for RustIdentifierChars {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        static CHARS: &[u8] = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();
        *CHARS.choose(rng).unwrap() as char
    }
}

impl SampleString for RustIdentifierChars {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        super::super::sample_append_string(RustIdentifierChars, rng, string, len);
    }
}
