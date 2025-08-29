use crate::docs;

#[doc = docs::side_by_side_md!("HasConstUsizeValue")]
pub trait HasConstUsizeValue {
    #[doc = docs::side_by_side_md!("HasConstUsizeValue.VALUE")]
    const VALUE: usize;
}

#[doc = docs::side_by_side_md!("ConstUsize")]
pub struct ConstUsize<const VALUE: usize>;

impl<const VALUE: usize> HasConstUsizeValue for ConstUsize<VALUE> {
    #[doc = docs::side_by_side_md!("ConstUsize.VALUE")]
    const VALUE: usize = VALUE;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use super::*;

    #[test]
    fn VALUE__get__expect_same_value_passed_as_generic_argument() {
        expect!(ConstUsize::<{usize::MIN}>::VALUE).to_equal(usize::MIN);
        expect!(ConstUsize::<1>::VALUE).to_equal(1);
        expect!(ConstUsize::<2376>::VALUE).to_equal(2376);
        expect!(ConstUsize::<{usize::MAX}>::VALUE).to_equal(usize::MAX);
    }
}
