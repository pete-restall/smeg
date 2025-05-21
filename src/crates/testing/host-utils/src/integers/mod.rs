use rand::Rng;
use rand::distr::uniform::{SampleRange, SampleUniform};

pub fn any_u8() -> u8 {
    any_within(0..=u8::MAX)
}

fn any_within<T: SampleUniform, R: SampleRange<T>>(bounds: R) -> T {
    let mut rng = rand::rng();
    rng.random_range(bounds)
}

pub fn any_usize() -> usize {
    any_within(0..=usize::MAX)
}

pub fn any_usize_within<R: SampleRange<usize>>(bounds: R) -> usize {
    any_within(bounds)
}
