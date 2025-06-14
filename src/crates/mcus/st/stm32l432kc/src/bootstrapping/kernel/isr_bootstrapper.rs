use core::marker::PhantomData;

use smeg_kernel::bootstrapping::kernel::IsrBootstrapping;
use smeg_kernel::interrupts::IsrContext;

pub struct IsrBootstrapper<C: IsrContext> {
    _unused: PhantomData<C>
}

impl<C: IsrContext> IsrBootstrapping for IsrBootstrapper<C> {
    type IsrContext = C;
}
