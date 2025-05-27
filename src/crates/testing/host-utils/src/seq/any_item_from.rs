use rand::seq::IndexedRandom;

pub fn any_item_from<T>(items: &[T]) -> &T {
    let mut rng = rand::rng();
    items.choose(&mut rng).unwrap()
}
