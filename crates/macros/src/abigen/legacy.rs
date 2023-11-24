//! Cairo 0 legacy support for abigen.
//!
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;

use std::collections::HashMap;

use starknet::core::types::contract::{
    legacy::RawLegacyAbiEntry, AbiNamedMember, AbiOutput, StateMutability,
};
use starknet_abigen_parser::cairo_types::CAIRO_BASIC_STRUCTS;
use starknet_abigen_parser::{CairoFunction, CairoStruct};

use crate::abigen::expand::contract::CairoContract;
use crate::abigen::expand::{Expandable, ExpandableFunction};

use crate::abigen::contract_abi::ContractAbiLegacy;
use crate::abigen::expand::utils;

pub fn abigen_internal_legacy(input: TokenStream) -> TokenStream {
    let contract_abi = parse_macro_input!(input as ContractAbiLegacy);
    let contract_name = contract_abi.name;
    let abi = contract_abi.abi;

    let mut tokens: Vec<TokenStream2> = vec![];

    tokens.push(CairoContract::expand(contract_name.clone()));

    let mut structs: HashMap<String, CairoStruct> = HashMap::new();
    let mut views = vec![];
    let mut views_reader = vec![];
    let mut externals = vec![];

    for entry in &abi {
        parse_entry(
            entry,
            &mut structs,
            &mut externals,
            &mut views,
            &mut views_reader,
        );
    }

    for (_, cs) in structs {
        tokens.push(cs.expand_decl(true));
        tokens.push(cs.expand_impl(true));
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
    entry: &RawLegacyAbiEntry,
    structs: &mut HashMap<String, CairoStruct>,
    externals: &mut Vec<TokenStream2>,
    views: &mut Vec<TokenStream2>,
    views_reader: &mut Vec<TokenStream2>,
) {
    match entry {
        RawLegacyAbiEntry::Struct(s) => {
            let members = s
                .members
                .iter()
                .map(|m| AbiNamedMember {
                    name: m.name.clone(),
                    r#type: m.r#type.clone(),
                })
                .collect();

            let cs = CairoStruct::new(&s.name, &members);

            if CAIRO_BASIC_STRUCTS.contains(&cs.get_name().as_str()) {
                return;
            }

            if let Some(ref mut existing_cs) = structs.get_mut(&cs.get_name()) {
                cs.compare_generic_types(existing_cs);
            } else {
                structs.insert(cs.get_name(), cs.clone());
            }
        }
        RawLegacyAbiEntry::Function(f) => {
            // Looks like in Cairo 0 ABI, if no mutability is given, it's an external.
            let mutability = match f.state_mutability {
                Some(_) => StateMutability::View,
                None => StateMutability::External,
            };

            let inputs: Vec<AbiNamedMember> = f
                .inputs
                .iter()
                .map(|i| AbiNamedMember {
                    name: i.name.clone(),
                    r#type: i.r#type.clone(),
                })
                .collect();

            let outputs: Vec<AbiOutput> = f
                .outputs
                .iter()
                .map(|o| AbiOutput {
                    r#type: o.r#type.clone(),
                })
                .collect();

            let cf = CairoFunction::new(&f.name, mutability.clone(), &inputs, &outputs);
            match mutability {
                StateMutability::View => {
                    views.push(cf.expand_impl(false, true));
                    views_reader.push(cf.expand_impl(true, true));
                }
                StateMutability::External => externals.push(cf.expand_impl(false, true)),
            }
        }
        // RawLegacyAbiEntry::Event(ev) => {
        //     if let Some(cev) = CairoEvent::new(ev) {
        //         events.push(cev);
        //     }
        // }
        _ => (),
    }
}
