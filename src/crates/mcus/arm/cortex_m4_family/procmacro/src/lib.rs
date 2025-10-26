use proc_macro::TokenStream;

mod arm_register;

mod datasheet_attribute;
use datasheet_attribute::*;

mod field_attribute;
use field_attribute::*;

mod register_attribute;
use register_attribute::*;

mod register_definition;

mod single;
use single::*;

#[proc_macro_attribute]
pub fn arm_register(args: TokenStream, input: TokenStream) -> TokenStream {
    arm_register::arm_register(args, input)
}
