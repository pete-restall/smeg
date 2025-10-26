use rand::RngCore;

pub fn any_bool() -> bool {
    let mut rng = rand::rng();
    (rng.next_u32() & 1) == 0
}
