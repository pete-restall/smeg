pub trait IsrBootstrapping {
    type IsrContext: crate::interrupts::IsrContext;
}
