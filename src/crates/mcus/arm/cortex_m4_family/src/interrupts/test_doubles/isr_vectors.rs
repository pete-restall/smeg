use smeg_testing_host_utils::seq::any_item_from;

use crate::interrupts::{IsrVector, IsrVectorTableBuilder};

use super::{Dummy, Stub};

// TODO: docs...
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
    panic!("Aborting because isr_stubbed_for_despair_1() should never be called");
}

unsafe extern "C" fn isr_stubbed_for_despair_2() -> ! {
    panic!("Aborting because isr_stubbed_for_despair_2() should never be called");
}

unsafe extern "C" fn isr_stubbed_for_despair_3() -> ! {
    panic!("Aborting because isr_stubbed_for_despair_3() should never be called");
}
