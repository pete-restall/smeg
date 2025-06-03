use smeg_kernel::despair;
use smeg_kernel::errors::KernelErrorCode;

pub type IsrVector = unsafe extern "C" fn() -> !;

#[repr(C)]
pub struct IsrVectorTable {
    reset_handler: Option<IsrVector>,
    pub nmi: Option<IsrVector>,
    pub hard_fault: Option<IsrVector>,
    pub mem_manage: Option<IsrVector>,
    pub bus_fault: Option<IsrVector>,
    pub usage_fault: Option<IsrVector>,
    _reserved_7: Option<IsrVector>,
    _reserved_8: Option<IsrVector>,
    _reserved_9: Option<IsrVector>,
    _reserved_10: Option<IsrVector>,
    pub sv_call: Option<IsrVector>,
    pub debug_monitor: Option<IsrVector>,
    _reserved_4: Option<IsrVector>,
    pub pend_sv: Option<IsrVector>,
    pub sys_tick: Option<IsrVector>
}

const _: () = {
    assert!(size_of::<IsrVectorTable>() == 15 * 4, "There must be 15 ISR vectors for the ARM Cortex M4, each 32-bits");
    assert!(align_of::<IsrVectorTable>() == 4, "Alignment of the ISR vector table must be 32-bits");
};

impl IsrVectorTable {
    pub const fn default() -> Self {
        unsafe extern "C" { unsafe fn _reset_handler() -> !; }
        Self {
            reset_handler: Some(_reset_handler),
            nmi: None,
            hard_fault: None,
            mem_manage: None,
            bus_fault: None,
            usage_fault: None,
            _reserved_7: RESERVED_ISR_VECTOR,
            _reserved_8: RESERVED_ISR_VECTOR,
            _reserved_9: RESERVED_ISR_VECTOR,
            _reserved_10: RESERVED_ISR_VECTOR,
            sv_call: None,
            debug_monitor: None,
            _reserved_4: RESERVED_ISR_VECTOR,
            pend_sv: None,
            sys_tick: None
        }
    }

    pub const fn with_default_unhandled(&self) -> Self {
        Self {
            nmi: with_default_unhandled(self.nmi),
            hard_fault: with_default_unhandled(self.hard_fault),
            mem_manage: with_default_unhandled(self.mem_manage),
            bus_fault: with_default_unhandled(self.bus_fault),
            usage_fault: with_default_unhandled(self.usage_fault),
            sv_call: with_default_unhandled(self.sv_call),
            debug_monitor: with_default_unhandled(self.debug_monitor),
            pend_sv: with_default_unhandled(self.pend_sv),
            sys_tick: with_default_unhandled(self.sys_tick),
            ..*self
        }
    }
}

pub const fn with_default_unhandled(isr: Option<IsrVector>) -> Option<IsrVector> {
    smeg_kernel::some_or(isr, UNHANDLED_ISR_VECTOR)
}

pub const UNHANDLED_ISR_VECTOR: Option<IsrVector> = Some(unhandled_isr);

unsafe extern "C" fn unhandled_isr() -> ! {
    despair!(
        with(KernelErrorCode::InsideUnhandledIsr),
        because("inside unhandled ISR; possibly a device enabled without a driver ?"));
}

pub const RESERVED_ISR_VECTOR: Option<IsrVector> = Some(reserved_isr);

unsafe extern "C" fn reserved_isr() -> ! {
    despair!(
        with(KernelErrorCode::InsideReservedIsr),
        because("inside reserved ISR; probably rogue software as the hardware shouldn't be capable ?"));
}
