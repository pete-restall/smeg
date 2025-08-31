pub fn array_filled_using<T, const N: usize, F: FnMut() -> T>(mut create: F) -> [T; N] {
    let mut vec = Vec::<T>::with_capacity(N);
    while vec.len() < N {
        vec.push(create());
    }
    vec.try_into().unwrap_or_else(|_| panic!("Vector size mismatch; this should never happen"))
}
