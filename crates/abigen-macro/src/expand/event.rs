//! Events expansion.
use crate::expand::generic;
use crate::expand::utils::{str_to_ident, str_to_type, str_to_litstr};
use crate::Expandable;

use cairo_type_parser::abi_types::{AbiType, AbiTypeAny};
use cairo_type_parser::cairo_event::{CairoEvent, CairoEventInner};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashMap;
use syn::Ident;

impl Expandable for CairoEvent {
    fn expand_decl(&self) -> TokenStream2 {
        let decl = match &self.inner {
            CairoEventInner::Struct(s) => s.expand_decl(),
            CairoEventInner::Enum(e) => e.expand_decl(),
        };
        quote!(#decl)
    }

    fn expand_impl(&self) -> TokenStream2 {
        let mut tokens = vec![];

        let inner_imp = match &self.inner {
            CairoEventInner::Struct(s) => s.expand_impl(),
            CairoEventInner::Enum(e) => e.expand_impl(),
        };

        tokens.push(quote!(#inner_imp));

        // Generate the get_selector() method for this event.
        let name_ident = str_to_ident(&self.get_name());
        let name_str = str_to_litstr(&self.get_name());
        let selector = quote! {
            impl #name_ident {
                pub fn get_selector() -> starknet::core::types::FieldElement {
                    starknet::macros::selector!(#name_str)
                }
            }
        };

        tokens.push(selector);

        // If it's the event enum with all other events (always called Event),
        // We can generate the TryFrom<EmittedEvent>. It's always an enum.
        // if self.get_name() == "Event" {
        //     // It should always be an enum here.
        //     if let CairoEventInner::Enum(inner) = self.inner {
        //         let variants_tokens = vec![];

        //         for (v_name, _) in inner.variants {
        //             let v_ident = str_to_ident(v_name);

        //             let tok = quote! {
        //                 if selector == #v_name::get_selector() {
        //                     return Ok(AnyEvent::MyEventA(MyEventA {
        //                         header: event.keys[1],
        //                         value: vec![event.data[0]],
        //                     }))
        //                 };
        //             }
        //         }

        //         let event_emitted_event = quote! {
        //             impl TryFrom<EmittedEvent> for AnyEvent {
        //                 // TODO: change for a better/custom type?
        //                 type Error = &'static str;

        //                 fn try_from(event: EmittedEvent) -> Result<Self, Self::Error> {
        //                     if event.keys.is_empty() {
        //                         return Err("Missing event selector, no keys found");
        //                     }

        //                     let selector = event.keys[0];


        //                     Err("Could not match any event selector")
        //                 }
        //             }
        //         }
        //     }
        // };


        quote!{
            #(#tokens)*
        }
    }
}
