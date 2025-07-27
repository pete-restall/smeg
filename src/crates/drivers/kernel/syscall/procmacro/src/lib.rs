use proc_macro::TokenStream;

mod syscall_args;

#[proc_macro_attribute]
pub fn syscall_args(args: TokenStream, items: TokenStream) -> TokenStream {
    syscall_args::syscall_args(args, items)
}
