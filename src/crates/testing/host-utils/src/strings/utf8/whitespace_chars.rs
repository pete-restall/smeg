use rand::Rng;
use rand::distr::{Distribution, SampleString};
use rand::seq::IndexedRandom;

pub struct WhitespaceChars;

impl Distribution<char> for WhitespaceChars {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        static WHITESPACE: [char; 25] = [
            '\u{0009}',
            '\u{000a}',
            '\u{000b}',
            '\u{000c}',
            '\u{000d}',
            '\u{0020}',
            '\u{0085}',
            '\u{00a0}',
            '\u{1680}',
            '\u{2000}',
            '\u{2001}',
            '\u{2002}',
            '\u{2003}',
            '\u{2004}',
            '\u{2005}',
            '\u{2006}',
            '\u{2007}',
            '\u{2008}',
            '\u{2009}',
            '\u{200a}',
            '\u{2028}',
            '\u{2029}',
            '\u{202f}',
            '\u{205f}',
            '\u{3000}'
        ];

        *WHITESPACE.choose(rng).unwrap_or(&' ')
    }
}

impl SampleString for WhitespaceChars {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        super::super::sample_append_string(WhitespaceChars, rng, string, len);
    }
}
