use proc_macro::TokenStream;

#[proc_macro]
pub fn dummy(_items: TokenStream) -> TokenStream {
    // TODO: Dummy (placeholder) procedural macro
    panic!("Dummy (placeholder) procedural macro called; nothing is implemented in kernel-procmacro yet !");
}
