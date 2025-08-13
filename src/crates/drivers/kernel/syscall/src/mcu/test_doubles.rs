pub struct IsrContext { }

impl smeg_kernel::interrupts::IsrContext for IsrContext { }

impl AsMut<IsrContext> for IsrContext {
    fn as_mut(&mut self) -> &mut IsrContext { self }
}

#[derive(Debug)]
pub struct IsrVectorTableBuilder;

pub const fn collect_isr_vectors<D>(isrs: IsrVectorTableBuilder) -> IsrVectorTableBuilder {
    return isrs;
}
