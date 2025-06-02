use crate::{caller, docs};

#[doc = docs::side_by_side_md!("PanicBootstrapping")]
pub trait PanicBootstrapping {
    #[doc = docs::side_by_side_md!("PanicBootstrapping.bootstrap")]
    fn bootstrap<K: caller::RestrictedToKernel>() {
        // TODO: implementation to be decided - we can call something in 'panic_handler.rs' that passes down a way to figure out the
        // current MCU core ID, the current task that's running on the core, etc.  Perhaps we need an associated PanicHandling trait that
        // the 'panic_handler.rs' can implement, and we call 'initialise()' on that.  All methods can be 'callable::RestrictedToKernel'
        // to ensure there is in fact only a single implementation (which can be public and thus used by smeg-os for injection).
    }
}

#[doc = docs::side_by_side_md!("PanicBootstrapping.DefaultPanicBootstrapper")]
pub struct DefaultPanicBootstrapper;
impl PanicBootstrapping for DefaultPanicBootstrapper { }

/*
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use super::*;

    #[test]
    fn bootstrap__called__expect_something() {
        ...
    }
}
*/
