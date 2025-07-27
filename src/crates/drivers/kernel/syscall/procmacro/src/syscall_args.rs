use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn syscall_args(_args: TokenStream, items: TokenStream) -> TokenStream {
    let original_items = proc_macro2::TokenStream::from(items.clone());
    let syscall_struct = syn::parse_macro_input!(items as DeriveInput);
    let args_ident = syscall_struct.ident;
    let syscall_name = args_ident.to_string();
    quote! {
        #original_items

        const _: () = {
            impl ::smeg_drivers_kernel_syscall::HasSyscallId for #args_ident {
                fn syscall_id() -> usize {
                    unsafe extern "Rust" {
                        #[link_name = concat!(".smeg.syscalls.isr_trampolines.", #syscall_name)]
                        static TRAMPOLINE: ::core::mem::MaybeUninit<usize>;
                    }

                    &raw const TRAMPOLINE as usize
                }
            }
        };
    }.into()
}
