use crate::HalfUsize;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KernelErrorCode {
    GeneralDespair,
    LinkerScriptDespair,
    BootstrappingPanic,
    InsideUnhandledIsr,
    InsideReservedIsr
}

impl From<KernelErrorCode> for HalfUsize {
    fn from(error: KernelErrorCode) -> Self {
        error as u8 as HalfUsize
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use super::*;

    static UNIT_VARIANTS: [(KernelErrorCode, HalfUsize); 5] = [
        (KernelErrorCode::GeneralDespair, 0),
        (KernelErrorCode::LinkerScriptDespair, 1),
        (KernelErrorCode::BootstrappingPanic, 2),
        (KernelErrorCode::InsideUnhandledIsr, 3),
        (KernelErrorCode::InsideReservedIsr, 4)];

    #[test]
    fn from__called_for_instance_of_each_unit_variant__expect_each_discriminator() {
        for unit in UNIT_VARIANTS {
            expect!(HalfUsize::from(unit.0)).to_equal(unit.1);
        }
    }

    #[test]
    fn into__called_for_instance_of_each_unit_variant__expect_each_discriminator() {
        for unit in UNIT_VARIANTS {
            expect!(Into::<HalfUsize>::into(unit.0)).to_equal(unit.1);
        }
    }
}
