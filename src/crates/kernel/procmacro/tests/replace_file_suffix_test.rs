#![allow(non_snake_case)]

use fluent_test::prelude::*;

use smeg_kernel_procmacro::replace_file_suffix;

macro_rules! replace_file_suffix__called__expect {
    (($old_suffix:literal, $new_suffix:literal) must be $expected:expr) => {
        expect!(replace_file_suffix!($old_suffix, $new_suffix)).to_equal($expected);
    };
}

#[test]
fn replace_file_suffix__called__expect_file_with_replaced_suffix() {
    replace_file_suffix__called__expect!((".rs", "") must be &file!().strip_suffix(".rs").unwrap());
    replace_file_suffix__called__expect!((".RS", ".something") must be file!());
    replace_file_suffix__called__expect!((".rs", ".whatever") must be &format!("{}.whatever", file!().strip_suffix(".rs").unwrap()));
    replace_file_suffix__called__expect!(("_test.rs", "") must be file!().strip_suffix("_test.rs").unwrap());
}
