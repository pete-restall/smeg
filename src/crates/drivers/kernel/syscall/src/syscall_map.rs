#[macro_export]
macro_rules! syscall_map {
    ($syscall_name:ident -> $handler:ty) => {
        const _: () = {
            use ::core::mem::{MaybeUninit, size_of};
            use ::smeg_drivers_kernel_syscall::isr::{SyscallIsrHandler, SyscallIsrTrampoline, SyscallIsrTrampolinePtr};

            type Handler = $handler;
            type IsrContext = <Handler as SyscallIsrHandler>::IsrContext;

            assert!(
                size_of::<MaybeUninit<usize>>() == size_of::<SyscallIsrTrampolinePtr<IsrContext>>(),
                "This code makes the assumption that a Syscall trampoline pointer is the same size as usize");

            #[used]
            #[unsafe(no_mangle)]
            #[unsafe(link_section = ".rodata.drivers.syscall.isr_trampolines.vector_table")]
            #[unsafe(export_name = concat!(".rodata.drivers.syscall.isr_trampolines.", stringify!($syscall_name)))]
            pub static TRAMPOLINE: SyscallIsrTrampolinePtr<IsrContext> = <Handler as SyscallIsrTrampoline<IsrContext>>::on_syscall;
        };
    };

    ($($syscall_name:ident -> $handler:ty);+) => {
        $(::smeg_drivers_kernel_syscall::syscall_map! { $syscall_name -> $handler })+
    };
}
