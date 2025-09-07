use rand::distr::uniform::SampleRange;

use crate::integers::any_usize_within;

pub fn any_vec_filled_using<R: SampleRange<usize>, T, F: FnMut() -> T>(length: R, create: F) -> Vec<T> {
    let length = any_usize_within(length);
    vec_filled_using(length, create)
}

pub fn vec_filled_using<T, F: FnMut() -> T>(length: usize, mut create: F) -> Vec<T> {
    let mut vec = Vec::<T>::with_capacity(length);
    while vec.len() < length {
        vec.push(create());
    }
    vec
}
