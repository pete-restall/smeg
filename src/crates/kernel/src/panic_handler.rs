use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO: the kernel will need to handle this, since the panic could be from a running task or from the kernel itself...both
    // require different behaviours (task can be aborted and resources reclaimed; kernel will perhaps reset or do some other
    // cfg-defined action)

    loop { }
}
