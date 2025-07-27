use smeg_kernel::bootstrapping::kernel::BoardMcuBootstrapping;

use smeg_mcu_st_stm32l432kc::define_isr_vector_table_from;
use smeg_mcu_st_stm32l432kc::interrupts::IsrVectorTableBuilder;

use crate::drivers::Drivers;

pub struct BoardMcuBootstrapper;
impl BoardMcuBootstrapping for BoardMcuBootstrapper { }

define_isr_vector_table_from!(Drivers::collect_isr_vectors(IsrVectorTableBuilder::default()));
