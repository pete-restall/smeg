use core::num::NonZero;

use crate::HalfUsize;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KernelErrorCode {
    GeneralDespair(u8) = 1,
    LinkerScriptDespair,
    BootstrappingPanic,
    InsideUnhandledIsr,
    InsideReservedIsr,
    Retryable(u8),
    UnknownSyscall,
    GeneralSyscallError(u8)
}

const _: () = {
    assert!(size_of::<KernelErrorCode>() <= size_of::<HalfUsize>(), "Size of KernelErrorCode must fit into half of a machine word");
};

impl From<KernelErrorCode> for NonZero<HalfUsize> {
    fn from(error: KernelErrorCode) -> Self {
        assert!(size_of::<KernelErrorCode>() == size_of::<u16>(), "Size of KernelErrorCode must be exactly 16 bits");
        unsafe {
            NonZero::new_unchecked(u16::from_be_bytes(*(&error as *const KernelErrorCode as *const [u8; 2])) as HalfUsize)
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;
    use smeg_testing_host_utils::integers::any_u8;

    use super::*;

    static UNIT_VARIANTS: [(KernelErrorCode, HalfUsize); 5] = [
        (KernelErrorCode::LinkerScriptDespair, 2 << 8),
        (KernelErrorCode::BootstrappingPanic, 3 << 8),
        (KernelErrorCode::InsideUnhandledIsr, 4 << 8),
        (KernelErrorCode::InsideReservedIsr, 5 << 8),
        (KernelErrorCode::UnknownSyscall, 7 << 8)];

    #[test]
    fn from__called_for_instance_of_each_unit_variant__expect_each_discriminator() {
        for unit in UNIT_VARIANTS {
            expect!(NonZero::<HalfUsize>::from(unit.0).get()).to_equal(unit.1);
        }
    }

    #[test]
    fn into__called_for_instance_of_each_unit_variant__expect_each_discriminator() {
        for unit in UNIT_VARIANTS {
            expect!(Into::<NonZero<HalfUsize>>::into(unit.0).get()).to_equal(unit.1);
        }
    }

    #[test]
    fn from__called_for_instance_of_each_non_unit_variant__expect_each_discriminator_as_bits_15_to_8_and_argument_as_bits_7_to_0() {
        for unit in all_non_unit_variants() {
            expect!(NonZero::<HalfUsize>::from(unit.0).get()).to_equal(unit.1);
        }
    }

    fn all_non_unit_variants() -> [(KernelErrorCode, HalfUsize); 3] {
        let any_u8 = any_u8();
        let any_u16 = any_u8 as u16;
        [
            (KernelErrorCode::GeneralDespair(any_u8), ((1_u16 << 8) | any_u16) as HalfUsize),
            (KernelErrorCode::Retryable(any_u8), ((6_u16 << 8) | any_u16) as HalfUsize),
            (KernelErrorCode::GeneralSyscallError(any_u8), ((8_u16 << 8) | any_u16) as HalfUsize)
        ]
    }

    #[test]
    fn into__called_for_instance_of_each_non_unit_variant__expect_each_discriminator_as_bits_15_to_8_and_argument_as_bits_7_to_0() {
        for unit in all_non_unit_variants() {
            expect!(Into::<NonZero<HalfUsize>>::into(unit.0).get()).to_equal(unit.1);
        }
    }
}

#[allow(unused)]
#[cfg(any(test, feature = "test_doubles"))]
pub mod test_doubles {
    use smeg_testing_host_utils::integers::any_u8;

    use super::KernelErrorCode;

    pub fn sample_of_all_kernel_error_codes() -> [KernelErrorCode; 8] {
        use KernelErrorCode::*;
        [
            GeneralDespair(any_u8()),
            LinkerScriptDespair,
            BootstrappingPanic,
            InsideUnhandledIsr,
            InsideReservedIsr,
            Retryable(any_u8()),
            UnknownSyscall,
            GeneralSyscallError(any_u8())
        ]
    }
}

pub mod prelude {
    pub use super::KernelErrorCode;
}
