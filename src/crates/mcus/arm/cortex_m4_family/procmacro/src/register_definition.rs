use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::BitXor;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

pub struct RegisterDefinitionGenerator<T: BitXor<Output = T> + Copy + Debug + PartialEq> {
    _type: PhantomData<T>
}

impl<T: BitXor<Output = T> + Copy + Debug + PartialEq> RegisterDefinitionGenerator<T> {
    pub fn generate(derive: &DeriveInput, type_ident: &Ident) -> Result<TokenStream, String> {
        let (visibility, register_ident) = (&derive.vis, &derive.ident);

        //let attrs = derive.attrs.iter().map(RegisterAttribute::<T>::parse).collect::<Result<Vec<_>>>();
        //err will panic
        //look for datasheet and extract that for docs; multiple datasheets panic
        //test all fields for (case-insensitive) unique names
        //test all fields for overlapping bits; else panic
        //test or'd fields to ensure all bits set; else panic
        //group all reserved fields by their type and or them
        //iterate fields below and build up consts, etc.
        //figure out if the register is readable (any ro|rw) and writable (any wo|wr)

        Ok(quote! {
            #[repr(transparent)]
            #[derive(Copy, Clone)]
            #visibility struct #register_ident(#type_ident);

            impl #register_ident {
                // also want some other const booleans - IS_READABLE, IS_WRITABLE, IS_READONLY, IS_WRITEONLY - but put these on a trait
                pub const FIELD_1_MASK: #type_ident = 1 << 31;
                pub const FIELD_1_MSB: #type_ident = 31;
                pub const FIELD_1_LSB: #type_ident = 31;
                pub const FIELD_1_WIDTH: #type_ident = 1;
            }

    //        impl<'mem> #accessor_name<'mem> {
                // TODO...can't use generics from 'input'...
    //    pub fn implementer(&self) -> usize { unsafe { self.accessor.mmio_read_masked_shifted_right::<{Cpuid::IMPLEMENTER_MASK}>() } }
    //     pub fn implementer_raw(&self) -> usize { unsafe { self.accessor.mmio_read_masked::<{Cpuid::IMPLEMENTER_MASK}>() } }

            // TODO: #visibility type {#register_name}Cell<M> = {Readonly|Writeonly|ReadWrite}Cell<M, #register_name>;
        })
    }
}
