// TODO: document !
pub trait HasIsrContext {
    type IsrContext: IsrContext;
}

// TODO: document !
pub trait HasFamilyIsrContext {
    type FamilyIsrContext: IsrContext;
}

// TODO: document !
pub trait HasInterruptedTask { // TODO: This trait does not belong here.  It should be defined in the 'scheduler' module, or possibly even a Scheduler (family) Driver
    type InterruptedTask;

    fn interrupted_task(&self) -> &Self::InterruptedTask;
}

pub trait HasInterruptedTaskMut: HasInterruptedTask {
    fn interrupted_task_mut(&mut self) -> &mut Self::InterruptedTask;
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

// TODO
//    impl HasInterruptedTask for Dummy {
//        type InterruptedTask = Dummy;
//    }
}
