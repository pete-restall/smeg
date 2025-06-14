use smeg_kernel::bootstrapping::kernel::BoardMcuBootstrapping;

use smeg_mcu_st_stm32l432kc::define_isr_vector_table_from;
use smeg_mcu_st_stm32l432kc::bootstrapping::kernel::IsrBootstrapper;
use smeg_mcu_st_stm32l432kc::interrupts::IsrVectorTableBuilder;

use crate::drivers::Drivers;


pub struct DummyIsrContext;
impl smeg_kernel::interrupts::IsrContext for DummyIsrContext {
// TODO: the whole IsrContext thing can be elaborated, hopefully with a generic parameter 'T' which each driver can inject on a per-ISR basis during
// the bootstrapping phase.  This means the ISR can utilise whatever state it stores in the context.  For now, just give it a dummy (here, as opposed
// to somewhere more meaningful).
}

pub struct BoardMcuBootstrapper;

impl BoardMcuBootstrapping for BoardMcuBootstrapper {
    type IsrBootstrapper = IsrBootstrapper<DummyIsrContext>;
}

type ForBoard = <BoardMcuBootstrapper as BoardMcuBootstrapping>::IsrBootstrapper;
define_isr_vector_table_from!(Drivers::collect_isr_vectors(IsrVectorTableBuilder::<ForBoard>::default()));
