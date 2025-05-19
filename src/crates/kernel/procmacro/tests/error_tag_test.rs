#![allow(non_snake_case)]

use fluent_test::prelude::*;

use smeg_kernel::HalfUsize;
use smeg_kernel_procmacro::error_tag;

#[test]
fn error_tag__called__expect_sequentially_increasing_ids() {
    let error_tags = vec![
        error_tag!("this is an error tag"),
        error_tag!("and ", "another", "error", "tag", "but", "with", 1, 2, 3, "to", "stringify", "and", "concat"),
        error_tag!("and another")
    ];

    let tag_ids: Vec<_> = error_tags.iter().map(Into::<HalfUsize>::into).collect();

    expect!(tag_ids[1]).to_equal(tag_ids[0] + 1);
    expect!(tag_ids[2]).to_equal(tag_ids[1] + 1);
}
