use rand::Rng;
use rand::distr::uniform::SampleRange;

pub fn any_usize() -> usize {
    any_usize_within(0..=usize::MAX)
}

pub fn any_usize_within<R: SampleRange<usize>>(bounds: R) -> usize {
    let mut rng = rand::rng();
    rng.random_range(bounds)
}
