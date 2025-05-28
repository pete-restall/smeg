use std::fmt::Debug;

use fluent_test::prelude::*;

pub trait EqualIteratorsMatchers<T: Iterator, U: Iterator<Item = T::Item>> where T::Item: PartialEq + Clone + Debug {
    fn to_equal_iterators(self, expected: &mut U) -> Self;
}

impl<T: Iterator + Clone, U: Iterator<Item = T::Item>> EqualIteratorsMatchers<T, U> for Assertion<T> where T::Item: PartialEq + Clone + Debug {
    fn to_equal_iterators(mut self, expected: &mut U) -> Self {
        // FIXME: would be nice to avoid the multiple allocations and clones, but this is simpler and adequate for now.
        // It would also be nice and cleaner to be able to compare iterators on the LHS with slices on the RHS and vice-versa
        let subject = self.value.clone().collect::<Vec<_>>();
        let expected = expected.collect::<Vec<_>>();
        let assertion = Assertion::new(subject, self.expr_str);
        let mut assertion = if self.negated {
            assertion.not().to_equal_collection(&expected)
        } else {
            assertion.to_equal_collection(&expected)
        };

        self.steps.push(assertion.steps.pop().unwrap());
        self
    }
}
