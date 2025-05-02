use rand::Rng;
use rand::distr::{Distribution, SampleString};

pub fn any_nonempty_utf8() -> String {
    let mut rng = rand::rng();
    let len = rng.random_range(1..=32);
    AnyUtf8.sample_string(&mut rng, len)
}

struct AnyUtf8;

impl Distribution<char> for AnyUtf8 {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        const NUM_SURROGATES: usize = 0xe000 - 0xd800;
        const NUM_VALID: usize = 0x110000 - NUM_SURROGATES;
        const LOW_RANGE_P: f64 = (0xd800 as f64) / (NUM_VALID as f64);
        char::from_u32(if rng.random_bool(LOW_RANGE_P) {
            rng.random_range(0..0xd800)
        } else {
            rng.random_range(0xe000..0x110000)
        }).unwrap()
    }
}

impl SampleString for AnyUtf8 {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        string.reserve(len);
        string.extend(rng.sample_iter(AnyUtf8).take(len));
    }
}
