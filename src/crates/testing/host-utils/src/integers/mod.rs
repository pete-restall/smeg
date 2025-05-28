use rand::Rng;
use rand::distr::uniform::{SampleRange, SampleUniform};

pub fn any_u8() -> u8 {
    any_within(0..=u8::MAX)
}

fn any_within<T: SampleUniform, R: SampleRange<T>>(bounds: R) -> T {
    let mut rng = rand::rng();
    rng.random_range(bounds)
}

pub fn any_u8_except(except: u8) -> u8 {
    any_except(except, any_u8)
}

fn any_except<T: SampleUniform + PartialEq>(except: T, any_value: fn() -> T) -> T {
    let value = any_value();
    if value != except {
        value
    } else {
        any_except(except, any_value)
    }
}

pub fn any_usize() -> usize {
    any_within(0..=usize::MAX)
}

pub fn any_usize_within<R: SampleRange<usize>>(bounds: R) -> usize {
    any_within(bounds)
}

pub fn any_usize_except(except: usize) -> usize {
    any_except(except, any_usize)
}
