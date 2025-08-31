use core::num::NonZero;

use crate::HalfUsize;

/*
TODO: Since KernelErrorCode is 16 bits and not all errors use the second byte, move the simple u8 errors such as LinkerScriptDespair, into their
own category (or categories).  If each top-level error in the enum has an argument then this increases the address space available for the errors.
Such categories would have their own enums.  Suggested layout:

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KernelErrorCode {
    GeneralDespair(u8) = 1,
    Retryable(u8),
    CoreError(CoreKernelErrorCode),
    McuError(McuKernelErrorCode),
    SyscallError(SyscallKernelErrorCode),
    InvalidSyscallArgs(u8),
    GeneralSyscallError(u8)
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CoreKernelErrorCode {
    LinkerScriptDespair,
    BootstrappingPanic
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum McuKernelErrorCode {
    InsideUnhandledIsr,
    InsideReservedIsr,
    InvalidCoreId
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SyscallKernelErrorCode {
    UnknownSyscall,
    UnalignedArgs,
    UnaddressableArgs
}
*/

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KernelErrorCode {
    GeneralDespair(u8) = 1,
    LinkerScriptDespair,
    BootstrappingPanic,
    InsideUnhandledIsr,
    InsideReservedIsr,
    InvalidMcuCoreId,
    Retryable(u8),
    UnknownSyscall,
    UnalignedSyscallArgs,
    UnaddressableSyscallArgs,
    InvalidSyscallArgs(u8),
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

    static UNIT_VARIANTS: [(KernelErrorCode, HalfUsize); 8] = [
        (KernelErrorCode::LinkerScriptDespair, 2 << 8),
        (KernelErrorCode::BootstrappingPanic, 3 << 8),
        (KernelErrorCode::InsideUnhandledIsr, 4 << 8),
        (KernelErrorCode::InsideReservedIsr, 5 << 8),
        (KernelErrorCode::InvalidMcuCoreId, 6 << 8),
        (KernelErrorCode::UnknownSyscall, 8 << 8),
        (KernelErrorCode::UnalignedSyscallArgs, 9 << 8),
        (KernelErrorCode::UnaddressableSyscallArgs, 10 << 8)
    ];

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

    fn all_non_unit_variants() -> [(KernelErrorCode, HalfUsize); 4] {
        let any_u8 = any_u8();
        let any_u16 = any_u8 as u16;
        [
            (KernelErrorCode::GeneralDespair(any_u8), ((1_u16 << 8) | any_u16) as HalfUsize),
            (KernelErrorCode::Retryable(any_u8), ((7_u16 << 8) | any_u16) as HalfUsize),
            (KernelErrorCode::InvalidSyscallArgs(any_u8), ((11_u16 << 8) | any_u16) as HalfUsize),
            (KernelErrorCode::GeneralSyscallError(any_u8), ((12_u16 << 8) | any_u16) as HalfUsize)
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
    use smeg_testing_host_utils::seq::any_item_from;

    use super::KernelErrorCode;

    pub fn any_kernel_error_code() -> KernelErrorCode {
        *any_item_from(&sample_of_all_kernel_error_codes())
    }

    pub fn sample_of_all_kernel_error_codes() -> [KernelErrorCode; 12] {
        use KernelErrorCode::*;
        [
            GeneralDespair(any_u8()),
            LinkerScriptDespair,
            BootstrappingPanic,
            InsideUnhandledIsr,
            InsideReservedIsr,
            InvalidMcuCoreId,
            Retryable(any_u8()),
            UnknownSyscall,
            UnalignedSyscallArgs,
            UnaddressableSyscallArgs,
            InvalidSyscallArgs(any_u8()),
            GeneralSyscallError(any_u8())
        ]
    }
}

pub mod prelude {
    pub use super::KernelErrorCode;
}
