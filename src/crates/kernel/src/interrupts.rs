// TODO: document !
pub trait HasIsrContext {
    type IsrContext: IsrContext;
}

// TODO: document !
pub trait HasFamilyIsrContext {
    type FamilyIsrContext: IsrContext;
}

// TODO: document !
pub trait IsrContext { }

// TODO: document !
pub struct NoIsrContext;
impl IsrContext for NoIsrContext { }

#[cfg(any(test, feature = "test_doubles"))]
pub mod test_doubles {
    use smeg_kernel::test_doubles::Dummy;

    use super::*;

    impl IsrContext for Dummy { }

    impl HasIsrContext for Dummy {
        type IsrContext = Dummy;
    }
}
