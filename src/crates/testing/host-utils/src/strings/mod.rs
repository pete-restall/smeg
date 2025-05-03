use rand::Rng;
use rand::distr::{Distribution, SampleString};
use rand::distr::uniform::SampleRange;

pub mod utf8;

pub(crate) fn any_string_of<D, R>(distribution: D, len: R) -> String where
    D: SampleString + Distribution<char>,
    R: SampleRange<usize> {

    let mut rng = rand::rng();
    let len = rng.random_range(len);
    distribution.sample_string(&mut rng, len)
}

pub(crate) fn sample_append_string<D, R>(distribution: D, rng: &mut R, string: &mut String, len: usize) where
    D: SampleString + Distribution<char>,
    R: Rng + ?Sized {

    string.reserve(len);
    string.extend(rng.sample_iter(distribution).take(len));
}
