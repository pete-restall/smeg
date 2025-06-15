use smeg_kernel::docs;

#[macro_export]
#[doc = docs::side_by_side_md!("isr_fn_trampolines")]
macro_rules! isr_fn_trampolines {
    // TODO: patterns for allowing multiple ISR trampolines to be defined

    ( fn $trampoline_fn_name:ident() -> $target_fn_name:ident<$($context_traits:ty),*>() -> "handler_main"; ) => {
        ::smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines! { @__( $trampoline_fn_name, $target_fn_name, ($($context_traits),*), 0xf1_u8 ) } /* TODO: 0xe1_u8 when using FP with extended frame... */
    };

    ( fn $trampoline_fn_name:ident() -> $target_fn_name:ident<$($context_traits:ty),*>() -> "thread_main"; ) => {
        ::smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines! { @__( $trampoline_fn_name, $target_fn_name, ($($context_traits),*), 0xf9_u8 ) } /* TODO: 0xe9_u8 when using FP with extended frame... */
    };

    ( fn $trampoline_fn_name:ident() -> $target_fn_name:ident<$($context_traits:ty),*>() -> "thread_process"; ) => {
        ::smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines! { @__( $trampoline_fn_name, $target_fn_name, ($($context_traits),*), 0xfd_u8 ) } /* TODO: 0xed_u8 when using FP with extended frame... */
    };

    ( @__(
        $trampoline_fn_name:ident,
        $target_fn_name:ident,
        ($($context_traits:ty),*),
        $fn_return:literal ) ) => {

        #[cfg_attr(target_arch = "arm", naked)]
        #[doc = "ARM Cortex M4 ISR Trampoline Stub - see [`isr_fn_trampoline!`] for details."]
        unsafe extern "C" fn $trampoline_fn_name<C>() -> !
            where C:
                ::smeg_kernel::interrupts::IsrContext +
                ::core::convert::From<::smeg_mcu_arm_cortex_m4_family::interrupts::IsrContextImpl> +
                ::core::borrow::BorrowMut<::smeg_mcu_arm_cortex_m4_family::interrupts::IsrContextImpl>
                $(+ $context_traits)* {

            ::cfg_if::cfg_if! {
                if #[cfg(target_arch = "arm")] {
                    unsafe extern "C" fn trampoline<C>(stack_frame: *mut ::smeg_mcu_arm_cortex_m4_family::interrupts::IsrBasicStackFrame)
                        where C:
                            ::smeg_kernel::interrupts::IsrContext +
                            ::core::convert::From<::smeg_mcu_arm_cortex_m4_family::interrupts::IsrContextImpl> +
                            ::core::borrow::BorrowMut<::smeg_mcu_arm_cortex_m4_family::interrupts::IsrContextImpl>
                            $(+ $context_traits)* {

                        let mut context = C::from(::smeg_mcu_arm_cortex_m4_family::interrupts::IsrContextImpl::from(stack_frame));
                        unsafe {
                            $target_fn_name(&mut context)
                        }
                    }

                    ::core::arch::naked_asm!(r#"
                        mov r0, sp
                        mvn lr, #{isr_retval}
                        b {trampoline}"#,
                        isr_retval = const !$fn_return,
                        trampoline = sym trampoline::<C>);

                } else {
                    panic!("Cannot call a Cortex M4 ISR trampoline on a non-Cortex M4 (running tests ?)");
                }
            }
        }
    };
}

pub mod prelude {
}
