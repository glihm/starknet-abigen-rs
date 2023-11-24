//! This crate contains all the logic to expand the parsed ABI types into
//! rust code.
//!
//! Important note, functions can't be generic when they are entry point
//! of a Cairo contracts.
//! For this reason, all the generic types are handles for structs and enums
//! generation only, and then applied on functions inputs/output.
//!
//! As the ABI as everything flatten, we must ensure that structs and enums are
//! checked for genericty to avoid duplicated types and detect correctly
//! the members/variants that are generic.
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;

use std::collections::HashMap;

use starknet::core::types::contract::{AbiEntry, StateMutability};
use starknet_abigen_parser::cairo_types::{CAIRO_BASIC_ENUMS, CAIRO_BASIC_STRUCTS};
use starknet_abigen_parser::{CairoEnum, CairoEvent, CairoFunction, CairoStruct};

mod expand;
use expand::contract::CairoContract;
use expand::{Expandable, ExpandableEvent, ExpandableFunction};

mod contract_abi;
use contract_abi::ContractAbi;

pub mod legacy;

use crate::abigen::expand::utils;

pub fn abigen_internal(input: TokenStream) -> TokenStream {
    let contract_abi = parse_macro_input!(input as ContractAbi);
    let contract_name = contract_abi.name;
    let abi = contract_abi.abi;

    let mut tokens: Vec<TokenStream2> = vec![];

    tokens.push(CairoContract::expand(contract_name.clone()));

    let mut structs: HashMap<String, CairoStruct> = HashMap::new();
    let mut enums: HashMap<String, CairoEnum> = HashMap::new();
    let mut views = vec![];
    let mut views_reader = vec![];
    let mut externals = vec![];
    let mut events = vec![];

    for entry in &abi {
        parse_entry(
            entry,
            &mut structs,
            &mut enums,
            &mut externals,
            &mut views,
            &mut views_reader,
            &mut events,
        );
    }

    for (_, cs) in structs {
        tokens.push(cs.expand_decl(false));
        tokens.push(cs.expand_impl(false));
    }

    for (_, ce) in enums {
        tokens.push(ce.expand_decl(false));
        tokens.push(ce.expand_impl(false));
    }

    for ev in &events {
        tokens.push(ev.expand_decl(false));
        tokens.push(ev.expand_impl(false, &events));
    }

    let reader = utils::str_to_ident(format!("{}Reader", contract_name).as_str());
    tokens.push(quote! {
        impl<A: starknet::accounts::ConnectedAccount + Sync> #contract_name<A> {
            #(#views)*
            #(#externals)*
        }

        impl<P: starknet::providers::Provider + Sync> #reader<P> {
            #(#views_reader)*
        }
    });

    let expanded = quote! {
        #(#tokens)*
    };

    expanded.into()
}

fn parse_entry(
    entry: &AbiEntry,
    structs: &mut HashMap<String, CairoStruct>,
    enums: &mut HashMap<String, CairoEnum>,
    externals: &mut Vec<TokenStream2>,
    views: &mut Vec<TokenStream2>,
    views_reader: &mut Vec<TokenStream2>,
    events: &mut Vec<CairoEvent>,
) {
    match entry {
        AbiEntry::Struct(s) => {
            let cs = CairoStruct::new(&s.name, &s.members);

            if CAIRO_BASIC_STRUCTS.contains(&cs.get_name().as_str()) {
                return;
            }

            if let Some(ref mut existing_cs) = structs.get_mut(&cs.get_name()) {
                cs.compare_generic_types(existing_cs);
            } else {
                structs.insert(cs.get_name(), cs.clone());
            }
        }
        AbiEntry::Enum(e) => {
            let ce = CairoEnum::new(&e.name, &e.variants);

            if CAIRO_BASIC_ENUMS.contains(&ce.get_name().as_str()) {
                return;
            }

            if let Some(ref mut existing_ce) = enums.get_mut(&ce.get_name()) {
                ce.compare_generic_types(existing_ce);
            } else {
                enums.insert(ce.get_name(), ce.clone());
            }
        }
        AbiEntry::Function(f) => {
            // Functions cannot be generic when they are entry point.
            // From this statement, we can safely assume that any function name is
            // unique.
            let cf = CairoFunction::new(&f.name, f.state_mutability.clone(), &f.inputs, &f.outputs);
            match f.state_mutability {
                StateMutability::View => {
                    views.push(cf.expand_impl(false, false));
                    views_reader.push(cf.expand_impl(true, false));
                }
                StateMutability::External => externals.push(cf.expand_impl(false, false)),
            }
        }
        AbiEntry::Event(ev) => {
            if let Some(cev) = CairoEvent::new(ev) {
                events.push(cev);
            }
        }
        AbiEntry::Interface(interface) => {
            for entry in &interface.items {
                parse_entry(
                    entry,
                    structs,
                    enums,
                    externals,
                    views,
                    views_reader,
                    events,
                );
            }
        }
        _ => (),
    }
}
