use smeg_kernel::despair;
use smeg_kernel::const_unwrap_or;
use smeg_kernel::errors::KernelErrorCode;

pub type IsrVector = unsafe extern "C" fn() -> !;

// TODO: Is this boilerplate a candidate for something like #[derive(IsrVectorTableBuilder)] ?
#[cfg_attr(feature = "test_doubles", derive(Clone, Debug, PartialEq))]
pub struct IsrVectorTableBuilder {
    pub nmi: Option<IsrVector>,
    pub hard_fault: Option<IsrVector>,
    pub mem_manage: Option<IsrVector>,
    pub bus_fault: Option<IsrVector>,
    pub usage_fault: Option<IsrVector>,
    pub sv_call: Option<IsrVector>,
    pub debug_monitor: Option<IsrVector>,
    pub pend_sv: Option<IsrVector>,
    pub sys_tick: Option<IsrVector>
}

impl IsrVectorTableBuilder {
    pub const fn default() -> Self {
        unsafe extern "C" { unsafe fn _reset_handler() -> !; }
        Self {
            nmi: None,
            hard_fault: None,
            mem_manage: None,
            bus_fault: None,
            usage_fault: None,
            sv_call: None,
            debug_monitor: None,
            pend_sv: None,
            sys_tick: None
        }
    }
}

#[repr(C)]
pub struct IsrVectorTable {
    reset_handler: IsrVector,
    pub nmi: IsrVector,
    pub hard_fault: IsrVector,
    pub mem_manage: IsrVector,
    pub bus_fault: IsrVector,
    pub usage_fault: IsrVector,
    _reserved_7: IsrVector,
    _reserved_8: IsrVector,
    _reserved_9: IsrVector,
    _reserved_10: IsrVector,
    pub sv_call: IsrVector,
    pub debug_monitor: IsrVector,
    _reserved_4: IsrVector,
    pub pend_sv: IsrVector,
    pub sys_tick: IsrVector
}

#[cfg(target_arch = "arm")]
const _: () = {
    assert!(size_of::<IsrVectorTable>() == 15 * 4, "There must be 15 ISR vectors for the ARM Cortex M4, each 32-bits");
    assert!(align_of::<IsrVectorTable>() == 4, "Alignment of the ISR vector table must be 32-bits");
};

impl IsrVectorTable {
    pub const fn from(isrs: IsrVectorTableBuilder) -> Self {
        unsafe extern "C" { unsafe fn _reset_handler() -> !; }
        Self {
            reset_handler: _reset_handler,
            nmi: const_unwrap_or(isrs.nmi, UNHANDLED_ISR_VECTOR),
            hard_fault: const_unwrap_or(isrs.hard_fault, UNHANDLED_ISR_VECTOR),
            mem_manage: const_unwrap_or(isrs.mem_manage, UNHANDLED_ISR_VECTOR),
            bus_fault: const_unwrap_or(isrs.bus_fault, UNHANDLED_ISR_VECTOR),
            usage_fault: const_unwrap_or(isrs.usage_fault, UNHANDLED_ISR_VECTOR),
            _reserved_7: RESERVED_ISR_VECTOR,
            _reserved_8: RESERVED_ISR_VECTOR,
            _reserved_9: RESERVED_ISR_VECTOR,
            _reserved_10: RESERVED_ISR_VECTOR,
            sv_call: const_unwrap_or(isrs.sv_call, UNHANDLED_ISR_VECTOR),
            debug_monitor: const_unwrap_or(isrs.debug_monitor, UNHANDLED_ISR_VECTOR),
            _reserved_4: RESERVED_ISR_VECTOR,
            pend_sv: const_unwrap_or(isrs.pend_sv, UNHANDLED_ISR_VECTOR),
            sys_tick: const_unwrap_or(isrs.sys_tick, UNHANDLED_ISR_VECTOR)
        }
    }
}

pub const UNHANDLED_ISR_VECTOR: IsrVector = unhandled_isr;

unsafe extern "C" fn unhandled_isr() -> ! {
    despair!(
        with(KernelErrorCode::InsideUnhandledIsr),
        because("inside unhandled ISR; possibly a device enabled without a driver ?"));
}

pub const RESERVED_ISR_VECTOR: IsrVector = reserved_isr;

unsafe extern "C" fn reserved_isr() -> ! {
    despair!(
        with(KernelErrorCode::InsideReservedIsr),
        because("inside reserved ISR; probably rogue software as the hardware shouldn't be capable ?"));
}

#[cfg(any(test, feature = "test_doubles"))]
pub mod test_doubles {
    use smeg_kernel::despair; // TODO: despair! should be re-exported from smeg_kernel::errors
    use smeg_kernel::errors::KernelErrorCode;

    use smeg_kernel::test_doubles::Stub;
    use smeg_testing_host_utils::seq::any_item_from;

    use super::*;

    impl From<Stub> for IsrVectorTableBuilder {
        fn from(_value: Stub) -> Self {
            Self {
                nmi: any_stub_isr(),
                hard_fault: any_stub_isr(),
                mem_manage: any_stub_isr(),
                bus_fault: any_stub_isr(),
                usage_fault: any_stub_isr(),
                sv_call: any_stub_isr(),
                debug_monitor: any_stub_isr(),
                pend_sv: any_stub_isr(),
                sys_tick: any_stub_isr(),
                ..Self::default()
            }
        }
    }

    fn any_stub_isr() -> Option<IsrVector> {
        const ISRS: [Option<IsrVector>; 4] = [
            None,
            Some(isr_stubbed_for_despair_1),
            Some(isr_stubbed_for_despair_2),
            Some(isr_stubbed_for_despair_3)
        ];

        *any_item_from(&ISRS)
    }

    unsafe extern "C" fn isr_stubbed_for_despair_1() -> ! {
        despair!(with(KernelErrorCode::GeneralDespair), because("Cortex M4 ISR stub was not expected to be called (1)"));
    }

    unsafe extern "C" fn isr_stubbed_for_despair_2() -> ! {
        despair!(with(KernelErrorCode::GeneralDespair), because("Cortex M4 ISR stub was not expected to be called (2)"));
    }

    unsafe extern "C" fn isr_stubbed_for_despair_3() -> ! {
        despair!(with(KernelErrorCode::GeneralDespair), because("Cortex M4 ISR stub was not expected to be called (3)"));
    }
}
