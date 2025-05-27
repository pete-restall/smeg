#![allow(non_snake_case)]

use fluent_test::prelude::*;

use smeg_kernel_procmacro::replace_suffix;

macro_rules! replace_suffix__called__expect {
    (($string:literal, $old_suffix:literal, $new_suffix:literal) must be $expected:expr) => {
        expect!(replace_suffix!($string, $old_suffix, $new_suffix)).to_equal($expected);
    };
}

#[test]
fn replace_suffix__called_with_empty_string__expect_empty_string() {
    replace_suffix__called__expect!(("", "a", "b") must be "");
    replace_suffix__called__expect!(("", "a", "") must be "");
    replace_suffix__called__expect!(("", "", "") must be "");
}

#[test]
fn replace_suffix__called_when_suffix_is_not_present__expect_same_string() {
    replace_suffix__called__expect!(("blah", "a", "b") must be "blah");
    replace_suffix__called__expect!(("a b", "a", "b") must be "a b");
    replace_suffix__called__expect!(("blah", "la", "b") must be "blah");
    replace_suffix__called__expect!(("blaH", "h", "b") must be "blaH");
}

#[test]
fn replace_suffix__called_when_suffix_is_present__expect_old_suffix_is_replaced_by_new_suffix() {
    replace_suffix__called__expect!(("blah", "h", "a") must be "blaa");
    replace_suffix__called__expect!(("blah", "h", "aaaah") must be "blaaaaah");
    replace_suffix__called__expect!(("blah", "blah", "a") must be "a");
    replace_suffix__called__expect!(("blah", "blah", "") must be "");
    replace_suffix__called__expect!(("blah", "h", "") must be "bla");
    replace_suffix__called__expect!(("", "", "hello") must be "hello");
    replace_suffix__called__expect!(("blah", "", "dy blah blah") must be "blahdy blah blah");
}
