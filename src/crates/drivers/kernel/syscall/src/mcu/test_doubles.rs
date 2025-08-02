pub trait IsrContext {
}

impl IsrContext for smeg_kernel::test_doubles::Dummy { }

#[derive(Debug)]
pub struct IsrVectorTableBuilder;

pub const fn collect_isr_vectors<D>(isrs: IsrVectorTableBuilder) -> IsrVectorTableBuilder {
	return isrs;
}
