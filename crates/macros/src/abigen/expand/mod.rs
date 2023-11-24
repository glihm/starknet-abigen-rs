pub(crate) mod contract;
pub(crate) mod r#enum;
pub(crate) mod event;
pub(crate) mod function;
pub(crate) mod generic;
pub(crate) mod r#struct;
pub(crate) mod utils;

use proc_macro2::TokenStream as TokenStream2;
use starknet_abigen_parser::CairoEvent;

pub trait Expandable {
    fn expand_decl(&self, is_legacy: bool) -> TokenStream2;
    fn expand_impl(&self, is_legacy: bool) -> TokenStream2;
}

pub trait ExpandableFunction {
    fn expand_decl(&self, is_legacy: bool) -> TokenStream2;
    fn expand_impl(&self, is_for_reader: bool, is_legacy: bool) -> TokenStream2;
}

pub trait ExpandableEvent {
    fn expand_decl(&self, is_legacy: bool) -> TokenStream2;
    fn expand_impl(&self, is_legacy: bool, events: &[CairoEvent]) -> TokenStream2;
}
