use smeg_kernel::{const_unwrap_or, despair};
use smeg_kernel::errors::KernelErrorCode;

pub type GeneralIsrVector = unsafe extern "C" fn() -> !;
pub type SvCallIsrVector = unsafe extern "C" fn(r0: usize, r1: usize, r2: usize, r3: usize) -> !;

// TODO: Is this boilerplate a candidate for something like #[derive(IsrVectorTableBuilder)] ?
#[cfg_attr(feature = "test_doubles", derive(Clone, Debug, PartialEq))]
pub struct IsrVectorTableBuilder {
    pub nmi: Option<GeneralIsrVector>,
    pub hard_fault: Option<GeneralIsrVector>,
    pub mem_manage: Option<GeneralIsrVector>,
    pub bus_fault: Option<GeneralIsrVector>,
    pub usage_fault: Option<GeneralIsrVector>,
    pub sv_call: Option<SvCallIsrVector>,
    pub debug_monitor: Option<GeneralIsrVector>,
    pub pend_sv: Option<GeneralIsrVector>,
    pub sys_tick: Option<GeneralIsrVector>
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
    reset_handler: GeneralIsrVector,
    pub nmi: GeneralIsrVector,
    pub hard_fault: GeneralIsrVector,
    pub mem_manage: GeneralIsrVector,
    pub bus_fault: GeneralIsrVector,
    pub usage_fault: GeneralIsrVector,
    _reserved_7: GeneralIsrVector,
    _reserved_8: GeneralIsrVector,
    _reserved_9: GeneralIsrVector,
    _reserved_10: GeneralIsrVector,
    pub sv_call: SvCallIsrVector,
    pub debug_monitor: GeneralIsrVector,
    _reserved_4: GeneralIsrVector,
    pub pend_sv: GeneralIsrVector,
    pub sys_tick: GeneralIsrVector
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
            sv_call: const_unwrap_or(isrs.sv_call, UNHANDLED_SV_CALL_ISR_VECTOR),
            debug_monitor: const_unwrap_or(isrs.debug_monitor, UNHANDLED_ISR_VECTOR),
            _reserved_4: RESERVED_ISR_VECTOR,
            pend_sv: const_unwrap_or(isrs.pend_sv, UNHANDLED_ISR_VECTOR),
            sys_tick: const_unwrap_or(isrs.sys_tick, UNHANDLED_ISR_VECTOR)
        }
    }
}

pub const UNHANDLED_ISR_VECTOR: GeneralIsrVector = unhandled_isr;

unsafe extern "C" fn unhandled_isr() -> ! {
    despair!(
        with(KernelErrorCode::InsideUnhandledIsr),
        because("inside unhandled ISR; possibly a device enabled without a driver ?"));
}

pub const RESERVED_ISR_VECTOR: GeneralIsrVector = reserved_isr;

unsafe extern "C" fn reserved_isr() -> ! {
    despair!(
        with(KernelErrorCode::InsideReservedIsr),
        because("inside reserved ISR; probably rogue software as the hardware shouldn't be capable ?"));
}

const UNHANDLED_SV_CALL_ISR_VECTOR: SvCallIsrVector = unhandled_sv_call_isr;

unsafe extern "C" fn unhandled_sv_call_isr(_r0: usize, _r1: usize, _r2: usize, _r3: usize) -> ! {
    unsafe { unhandled_isr() }
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
                sv_call: any_stub_sv_call_isr(),
                debug_monitor: any_stub_isr(),
                pend_sv: any_stub_isr(),
                sys_tick: any_stub_isr(),
                ..Self::default()
            }
        }
    }

    fn any_stub_isr() -> Option<GeneralIsrVector> {
        const ISRS: [Option<GeneralIsrVector>; 4] = [
            None,
            Some(isr_stubbed_for_despair_1),
            Some(isr_stubbed_for_despair_2),
            Some(isr_stubbed_for_despair_3)
        ];

        *any_item_from(&ISRS)
    }

    unsafe extern "C" fn isr_stubbed_for_despair_1() -> ! {
        despair!(with(KernelErrorCode::GeneralDespair(1)), because("Cortex M4 ISR stub was not expected to be called (1)"));
    }

    unsafe extern "C" fn isr_stubbed_for_despair_2() -> ! {
        despair!(with(KernelErrorCode::GeneralDespair(2)), because("Cortex M4 ISR stub was not expected to be called (2)"));
    }

    unsafe extern "C" fn isr_stubbed_for_despair_3() -> ! {
        despair!(with(KernelErrorCode::GeneralDespair(3)), because("Cortex M4 ISR stub was not expected to be called (3)"));
    }

    fn any_stub_sv_call_isr() -> Option<SvCallIsrVector> {
        const ISRS: [Option<SvCallIsrVector>; 2] = [
            None,
            Some(sv_call_isr_stubbed_for_despair)
        ];

        *any_item_from(&ISRS)
    }

    unsafe extern "C" fn sv_call_isr_stubbed_for_despair(_r0: usize, _r1: usize, _r2: usize, _r3: usize) -> ! {
        despair!(with(KernelErrorCode::GeneralDespair(0)), because("Cortex M4 SV_CALL ISR stub was not expected to be called"));
    }
}
