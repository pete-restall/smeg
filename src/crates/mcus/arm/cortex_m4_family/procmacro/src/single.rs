pub trait Single {
    type Item;

    fn single(self) -> Result<Self::Item, ()>;
    fn single_or_none(self) -> Result<Option<Self::Item>, ()>;
}

impl<I> Single for I where I: Iterator {
    type Item = I::Item;

    fn single(self) -> Result<Self::Item, ()> {
        self.single_or_none()?.ok_or(())
    }

    fn single_or_none(self) -> Result<Option<Self::Item>, ()> {
        match self.take(2).fold((None, 0), |only_one, item| (Some(item), only_one.1 + 1)) {
            (item, 1) => Ok(item),
            (_, 0) => Ok(None),
            _ => Err(())
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use smeg_testing_host_utils::integers::{any_isize, any_usize};

    use super::*;

    #[test]
    fn single__called_when_iterator_has_no_items__expect_err() {
        let empty = [0; 0];
        expect!(empty.iter().single()).to_be_err();
    }

    #[test]
    fn single__called_when_iterator_has_single_item__expect_same_item() {
        let single_item = [any_usize()];
        expect!(single_item.iter().single().unwrap()).to_equal(single_item[0]);
    }

    #[test]
    fn single__called_when_iterator_has_two_items__expect_err() {
        let two_items = [any_isize(), any_isize()];
        expect!(two_items.iter().single()).to_be_err();
    }

    #[test]
    fn single_or_none__called_when_iterator_has_no_items__expect_none() {
        let empty = [0; 0];
        expect!(empty.iter().single_or_none().unwrap()).to_be_none();
    }

    #[test]
    fn single_or_none__called_when_iterator_has_single_item__expect_same_item() {
        let single_item = [any_isize()];
        expect!(single_item.iter().single_or_none().unwrap().unwrap()).to_equal(single_item[0]);
    }

    #[test]
    fn single_or_none__called_when_iterator_has_two_items__expect_err() {
        let two_items = [any_usize(), any_usize()];
        expect!(two_items.iter().single_or_none()).to_be_err();
    }
}
