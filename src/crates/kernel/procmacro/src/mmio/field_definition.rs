use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::BitXor;

use proc_macro2::TokenStream;
use quote::quote;

use super::RegisterFieldAttribute;

pub struct RegisterFieldDefinition<'a, T> where T: BitXor<Output = T> + Copy + Debug + PartialEq {
	_todo: &'a PhantomData<T>
}

impl<'a, T> RegisterFieldDefinition<'a, T> where T: BitXor<Output = T> + Copy + Debug + PartialEq {
	pub fn parse(attribute: &RegisterFieldAttribute<T>) -> Result<Self, String> {
		Ok(Self { _todo: &PhantomData })
	}

	pub fn generate(&self) -> TokenStream {
		quote! { /* TODO */ }
	}
}
