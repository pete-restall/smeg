mod runtime_bootstrapping;
pub use runtime_bootstrapping::*;

pub unsafe fn initialise<R: RuntimeBootstrapping>() {
    // TODO: .bss, .data, .init, etc.
}
