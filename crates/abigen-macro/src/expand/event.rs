use crate::Expandable;
use cairo_type_parser::CairoEvent;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

// TODO: implement CairoEvent
impl Expandable for CairoEvent {
    fn expand_decl(&self) -> TokenStream2 {
        quote!()
    }

    fn expand_impl(&self) -> TokenStream2 {
        quote!()
    }
}
