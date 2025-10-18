use core::marker::PhantomData;

use crate::docs;

use super::Bank;

#[doc = docs::side_by_side_md!("BankAccessor")]
pub struct BankAccessor<'mem, B: Bank> {
    bank_ptr: *mut B,
    _memory_lifetime: PhantomData<&'mem B>
}

impl<'mem, B: Bank> BankAccessor<'mem, B> {
    #[doc = docs::side_by_side_md!("BankAccessor.new")]
    pub const unsafe fn new(bank_ptr: *mut B) -> Self {
        Self { bank_ptr, _memory_lifetime: PhantomData }
    }
}

pub mod prelude {
    pub use super::BankAccessor;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use crate::test_doubles::Dummy;

    use super::*;

    #[test]
    fn bank_ptr__get__expect_same_value_passed_to_constructor() {
        let mut bank = Dummy;
        let accessor = unsafe { BankAccessor::new(&raw mut bank) };
        expect!(accessor.bank_ptr).to_equal(&raw mut bank);
    }
}
