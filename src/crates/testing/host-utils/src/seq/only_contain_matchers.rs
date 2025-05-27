use std::fmt::Debug;

use fluent_test::backend::AssertionSentence;
use fluent_test::prelude::*;

pub trait OnlyContainNonCopyableMatchers<T: Iterator + Clone> where T::Item: PartialEq + Debug {
    fn to_only_contain(self, expected: &T::Item) -> Self;
    fn to_be_empty_or_only_contain(self, expected: &T::Item) -> Self;
}

impl<T: Iterator + Clone> OnlyContainNonCopyableMatchers<T> for Assertion<T> where T::Item: PartialEq + Debug {
    fn to_only_contain(mut self, expected: &T::Item) -> Self {
        let was_successful = only_contains(&mut self, |x| x == expected, false);
        let sentence = AssertionSentence::new("contain", format!("only {:?}", expected));
        self.add_step(sentence, was_successful)
    }

    fn to_be_empty_or_only_contain(mut self, expected: &T::Item) -> Self {
        let was_successful = only_contains(&mut self, |x| x == expected, true);
        let sentence = AssertionSentence::new("be", format!("empty or contain only {:?}", expected));
        self.add_step(sentence, was_successful)
    }
}

fn only_contains<T: Iterator + Clone, P>(assertion: &mut Assertion<T>, mut predicate: P, is_empty_ok: bool) -> bool
    where P: FnMut(&T::Item) -> bool {

    let mut only_contains: Option<bool> = None;
    for ref item in &mut assertion.value {
        only_contains = Some(predicate(item));
        if let Some(false) = only_contains {
            break;
        }
    }

    !assertion.negated && match only_contains {
        Some(only_contains) => only_contains,
        None => is_empty_ok
    }
}

pub trait OnlyContainCopyableMatchers<T: Iterator + Clone> where T::Item: Copy + PartialEq + Debug {
    fn to_only_contain(self, expected: T::Item) -> Self;
    fn to_be_empty_or_only_contain(self, expected: T::Item) -> Self;
}

impl<T: Iterator + Clone> OnlyContainCopyableMatchers<T> for Assertion<T> where T::Item: Copy + PartialEq + Debug {
    fn to_only_contain(mut self, expected: T::Item) -> Self {
        let was_successful = only_contains(&mut self, |x| *x == expected, false);
        let sentence = AssertionSentence::new("contain", format!("only {:?}", expected));
        self.add_step(sentence, was_successful)
    }

    fn to_be_empty_or_only_contain(mut self, expected: T::Item) -> Self {
        let was_successful = only_contains(&mut self, |x| *x == expected, true);
        let sentence = AssertionSentence::new("be", format!("empty or contain only {:?}", expected));
        self.add_step(sentence, was_successful)
    }
}

pub trait OnlyContainMatchingNonCopyableMatchers<T: Iterator + Clone> {
    fn to_only_contain_matching<P: FnMut(&T::Item) -> bool + Debug>(self, predicate: P) -> Self;
    fn to_be_empty_or_only_contain_matching<P: FnMut(&T::Item) -> bool + Debug>(self, predicate: P) -> Self;
}

impl<T: Iterator + Clone> OnlyContainMatchingNonCopyableMatchers<T> for Assertion<T> {
    fn to_only_contain_matching<P: FnMut(&T::Item) -> bool + Debug>(mut self, mut predicate: P) -> Self {
        let was_successful = only_contains(&mut self, |x| predicate(x), false);
        let sentence = AssertionSentence::new("contain", format!("only items matching {:?}", predicate));
        self.add_step(sentence, was_successful)
    }

    fn to_be_empty_or_only_contain_matching<P: FnMut(&T::Item) -> bool + Debug>(mut self, mut predicate: P) -> Self {
        let was_successful = only_contains(&mut self, |x| predicate(x), true);
        let sentence = AssertionSentence::new("be", format!("empty or contain only items matching {:?}", predicate));
        self.add_step(sentence, was_successful)
    }
}

pub trait OnlyContainMatchingCopyableMatchers<T: Iterator + Clone> where T::Item: Copy {
    fn to_only_contain_matching<P: FnMut(T::Item) -> bool + Debug>(self, predicate: P) -> Self;
    fn to_be_empty_or_only_contain_matching<P: FnMut(T::Item) -> bool + Debug>(self, predicate: P) -> Self;
}

impl<T: Iterator + Clone> OnlyContainMatchingCopyableMatchers<T> for Assertion<T> where T::Item: Copy {
    fn to_only_contain_matching<P: FnMut(T::Item) -> bool + Debug>(mut self, mut predicate: P) -> Self {
        let was_successful = only_contains(&mut self, |x| predicate(*x), false);
        let sentence = AssertionSentence::new("contain", format!("only items matching {:?}", predicate));
        self.add_step(sentence, was_successful)
    }

    fn to_be_empty_or_only_contain_matching<P: FnMut(T::Item) -> bool + Debug>(mut self, mut predicate: P) -> Self {
        let was_successful = only_contains(&mut self, |x| predicate(*x), true);
        let sentence = AssertionSentence::new("be", format!("empty or contain only items matching {:?}", predicate));
        self.add_step(sentence, was_successful)
    }
}
