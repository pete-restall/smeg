use core::panic::PanicInfo;

#[cfg(all(not(test), feature = "std"))]
pub fn claim_std_panic_hook() {
    std::panic::set_hook(Box::new(|_info| on_panic()));
}

fn on_panic() -> ! {
    // TODO: the kernel will need to handle this, since the panic could be from a running task or from the kernel itself...both
    // require different behaviours (task can be aborted and resources reclaimed; kernel will perhaps reset or do some other
    // cfg-defined action)

    loop { }
}

#[allow(dead_code)]
#[cfg_attr(not(any(test, feature = "std")), panic_handler)]
fn on_core_panic(_info: &PanicInfo) -> ! {
    on_panic();
}
