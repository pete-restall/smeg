use core::panic::PanicInfo;

use crate::despair;
use crate::errors::KernelErrorCode;

#[cfg(all(not(test), feature = "std"))]
pub fn claim_std_panic_hook() {
    std::panic::set_hook(Box::new(|_info| on_panic()));
}

fn on_panic() -> ! {
    if !super::is_rust_runtime_initialised() {
        despair!(
            with(KernelErrorCode::BootstrappingPanic),
            because("panic handling is provided by the Rust runtime and that is initialised during bootstrapping"));
    }

    // TODO: the kernel will need to handle this, since the panic could be from a running task or from the kernel itself...both
    // require different behaviours (task can be aborted and resources reclaimed; kernel will perhaps reset the device or restart
    // itself on the same core or do some other cfg-defined action).  We also need something initialising that lets us get at the
    // core-specific structures (ie. having a HasMcuCoreId injected would be a good start, perhaps as part of bootstrapping), since
    // that's where the Task Control Block will be kept (amongst other things) that allows us to determine if a userspace or kernel
    // task caused this.

    loop { }
}

#[allow(dead_code)]
#[cfg_attr(not(any(test, feature = "std")), panic_handler)]
fn on_core_panic(_info: &PanicInfo) -> ! {
    on_panic();
}
