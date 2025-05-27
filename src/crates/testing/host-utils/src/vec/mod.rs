use rand::distr::uniform::SampleRange;

use crate::integers::any_usize_within;

pub fn any_vec_filled_using<R: SampleRange<usize>, T>(length: R, create: fn() -> T) -> Vec<T> {
    let length = any_usize_within(length);
    let mut vec = Vec::<T>::with_capacity(length);
    while vec.len() < length {
        vec.push(create());
    }
    vec
}
