use crate::docs;

#[doc = docs::side_by_side_md!("HasMcuCoreId")]
pub trait HasMcuCoreId {
    #[doc = docs::side_by_side_md!("HasMcuCoreId.core_id")]
    fn core_id() -> usize;
}

#[doc = docs::side_by_side_md!("McuSingleCore")]
pub struct McuSingleCore;

impl HasMcuCoreId for McuSingleCore {
    #[doc = docs::side_by_side_md!("McuSingleCore.core_id")]
    fn core_id() -> usize { 0 }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use super::*;

    #[test]
    fn core_id__called__expect_hard_coded_zero() {
        expect!(McuSingleCore::core_id()).to_equal(0);
    }
}
