use crate::Expandable;

use cairo_type_parser::abi_type::AbiType;
use cairo_type_parser::{CairoAbiEntry, CairoStruct};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl Expandable for CairoStruct {
    fn expand_decl() -> TokenStream2 {
        quote!()
    }

    fn expand_impl() -> TokenStream2 {
        quote!()
    }
}
