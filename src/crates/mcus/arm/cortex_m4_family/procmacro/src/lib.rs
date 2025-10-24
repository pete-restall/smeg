use proc_macro::TokenStream;

mod arm_register;

mod datasheet_attribute;
use datasheet_attribute::*;

mod register_attribute;
mod register_definition;

#[proc_macro_attribute]
pub fn arm_register(args: TokenStream, input: TokenStream) -> TokenStream {
    arm_register::arm_register(args, input)
}
