use core::borrow::BorrowMut;
use core::convert::From;

use smeg_kernel::docs;
use smeg_kernel::interrupts::IsrContext;

pub trait IsrTrampolineContext: IsrContext + From<super::IsrContextImpl> + BorrowMut<super::IsrContextImpl> { }
impl<T: IsrContext + From<super::IsrContextImpl> + BorrowMut<super::IsrContextImpl>> IsrTrampolineContext for T { }

#[macro_export]
#[doc = docs::side_by_side_md!("isr_fn_trampolines")]
macro_rules! isr_fn_trampolines {
    // TODO: patterns for allowing multiple ISR trampolines to be defined

    ( fn $trampoline_fn_name:ident<$context_trait:path $(| $context_traits:path)*>() -> $target_fn_name:ident() -> "handler_main"; ) => {
        ::smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines! { @__( $trampoline_fn_name, ($context_trait $(| $context_traits)*), $target_fn_name, (), 0xf1_u8 ) } /* TODO: 0xe1_u8 when using FP with extended frame... */
    };

    ( fn $trampoline_fn_name:ident<$context_trait:path $(| $context_traits:path)*>() -> $target_fn_name:ident() -> "thread_main"; ) => {
        ::smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines! { @__( $trampoline_fn_name, ($context_trait $(| $context_traits)*), $target_fn_name, (), 0xf9_u8 ) } /* TODO: 0xe9_u8 when using FP with extended frame... */
    };

    ( fn $trampoline_fn_name:ident<$context_trait:path $(| $context_traits:path)*>() -> $target_fn_name:ident() -> "thread_process"; ) => {
        ::smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines! { @__( $trampoline_fn_name, ($context_trait $(| $context_traits)*), $target_fn_name, (), 0xfd_u8 ) } /* TODO: 0xed_u8 when using FP with extended frame... */
    };

    ( fn $trampoline_fn_name:ident<$context_trait:path $(| $context_traits:path)*>() -> $target_fn_name:ident<$($fn_generics:ty),+>() -> "handler_main"; ) => {
        ::smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines! { @__( $trampoline_fn_name, ($context_trait $(| $context_traits)*), $target_fn_name, ($($fn_generics),+), 0xf1_u8 ) } /* TODO: 0xe1_u8 when using FP with extended frame... */
    };

    ( fn $trampoline_fn_name:ident<$context_trait:path $(| $context_traits:path)*>() -> $target_fn_name:ident<$($fn_generics:ty),+>() -> "thread_main"; ) => {
        ::smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines! { @__( $trampoline_fn_name, ($context_trait $(| $context_traits)*), $target_fn_name, ($($fn_generics),+), 0xf9_u8 ) } /* TODO: 0xe9_u8 when using FP with extended frame... */
    };

    ( fn $trampoline_fn_name:ident<$context_trait:path $(| $context_traits:path)*>() -> $target_fn_name:ident<$($fn_generics:ty),+>() -> "thread_process"; ) => {
        ::smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines! { @__( $trampoline_fn_name, ($context_trait $(| $context_traits)*), $target_fn_name, ($($fn_generics),+), 0xfd_u8 ) } /* TODO: 0xed_u8 when using FP with extended frame... */
    };

    ( @__(
        $trampoline_fn_name:ident,
        ($context_trait:path $(| $context_traits:path)*),
        $target_fn_name:ident,
        ($($fn_generics:ty),*),
        $fn_return:literal ) ) => {

        #[cfg_attr(target_arch = "arm", naked)]
        #[doc = "ARM Cortex M4 ISR Trampoline Stub - see [`isr_fn_trampoline!`] for details."]
        unsafe extern "C" fn $trampoline_fn_name<D>() -> !
            where
                D: $context_trait $(+ $context_traits)*,
                D::IsrContext: ::smeg_mcu_arm_cortex_m4_family::interrupts::IsrTrampolineContext {

            unsafe extern "C" fn trampoline<D>(stack_frame: *mut ::smeg_mcu_arm_cortex_m4_family::interrupts::IsrBasicStackFrame)
                where
                    D: $context_trait $(+ $context_traits)*,
                    D::IsrContext: ::smeg_mcu_arm_cortex_m4_family::interrupts::IsrTrampolineContext {

                let mut context = <D::IsrContext>::from(::smeg_mcu_arm_cortex_m4_family::interrupts::IsrContextImpl::from(stack_frame));
                unsafe {
                    $target_fn_name::<D $(, $fn_generics)*>(&mut context)
                }
            }

            ::cfg_if::cfg_if! {
                if #[cfg(target_arch = "arm")] {
                    ::core::arch::naked_asm!(r#"
                        mov r0, sp
                        mvn lr, #{isr_retval}
                        b {trampoline}"#,
                        isr_retval = const !$fn_return,
                        trampoline = sym trampoline::<D>);

                } else {
                    panic!("Cannot call a Cortex M4 ISR trampoline on a non-Cortex M4 (running tests ?)");
                }
            }
        }
    };
}

pub mod prelude {
    pub use super::IsrTrampolineContext;
}
