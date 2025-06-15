use crate::HasMcuCoreId;

// TODO: document !
pub trait IsrContext {
    type Mcu: HasMcuCoreId;
}

#[cfg(any(test, feature = "test_doubles"))]
pub mod test_doubles {
    use smeg_kernel::test_doubles::Dummy;

    use super::*;

    impl IsrContext for Dummy {
        type Mcu = Dummy;
    }
}
