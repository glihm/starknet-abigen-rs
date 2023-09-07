use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use serde_json;
use starknet::accounts::Call;
use starknet::core::types::contract::*;
use starknet::core::types::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use syn::{
    braced,
    ext::IdentExt,
    parenthesized,
    parse::{Error, Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, LitInt, LitStr, Token, Type,
};

use cairo_type_parser::{abi_type::AbiType, CairoEnum};
use cairo_type_parser::{CairoAbiEntry, CairoStruct, CairoFunction};

mod expand;

trait Expandable {
    fn expand_decl(&self) -> TokenStream2;
    fn expand_impl(&self) -> TokenStream2;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ContractAbi {
    name: Ident,
    abi_file: LitStr,
}

impl Parse for ContractAbi {
    fn parse(input: ParseStream) -> Result<Self> {
        // name
        let name = input.parse::<Ident>()?;

        input.parse::<Token![,]>()?;

        // abi (from ether-rs crate).
        // Due to limitation with the proc-macro Span API, we
        // can't currently get a path the the file where we were called from;
        // therefore, the path will always be rooted on the cargo manifest
        // directory. Eventually we can use the `Span::source_file` API to
        // have a better experience.
        let abi_file = input.parse::<LitStr>()?;

        Ok(ContractAbi { name, abi_file })
    }
}

#[proc_macro]
pub fn abigen(input: TokenStream) -> TokenStream {
    let contract_abi = parse_macro_input!(input as ContractAbi);
    let file_path = contract_abi.abi_file.value();
    let contract_name = contract_abi.name;

    let abi_str = fs::read_to_string(file_path).expect("Can't load abi file");

    let abi: Vec<AbiEntry> =
        serde_json::from_str(&abi_str).expect("Json is not formatted as expected");

    let mut tokens: Vec<TokenStream2> = vec![];

    tokens.push(quote! {
        #[derive(Debug)]
        pub struct #contract_name {
            address: starknet::core::types::FieldElement,
            provider: starknet::providers::AnyProvider,
        }

        impl #contract_name {
            fn new(address: starknet::core::types::FieldElement,
                   provider: starknet::providers::AnyProvider
            ) -> #contract_name {
                #contract_name {
                    address,
                    provider,
                }
            }
        }
    });

    let mut functions = vec![];

    for entry in abi {
        match entry {
            AbiEntry::Struct(s) => {
                // Span is skipped as represented as Vec.
                // TODO: also skip the u256 which should be implemented
                // directly in cairo type.
                if s.name.contains("array::Span") {
                    continue;
                }

                let cairo_entry = CairoStruct {
                    name: AbiType::from_string(&s.name),
                    members: s
                        .members
                        .iter()
                        .map(|m| (m.name.clone(), AbiType::from_string(&m.r#type)))
                        .collect(),
                };

                tokens.push(cairo_entry.expand_decl());
                tokens.push(cairo_entry.expand_impl());
            }
            AbiEntry::Function(f) => {
                // Outputs is usually only one type. It's called "outputs"
                // to be generic. So for now we only keep the first output type (if any).
                //
                // TODO: ask to Starkware if there is a case where several outputs
                // are returned. As the functions only have one output type (which can be
                // nested, of course).
                let output;
                if f.outputs.len() > 0 {
                    output = Some(AbiType::from_string(&f.outputs[0].r#type));
                } else {
                    output = None;
                }

                let cairo_entry = CairoFunction {
                    name: AbiType::from_string(&f.name),
                    state_mutability: f.state_mutability,
                    inputs: f
                        .inputs
                        .iter()
                        .map(|i| (i.name.clone(), AbiType::from_string(&i.r#type)))
                        .collect(),
                    output,
                };

                functions.push(cairo_entry.expand_impl());
            }
            AbiEntry::Enum(e) => {
                // TODO: also skip Option, Result and other
                // very basic enums of Cairo that must be implemented
                // directly in CairoType.
                let cairo_entry = CairoEnum {
                    name: AbiType::from_string(&e.name),
                    variants: e
                        .variants
                        .iter()
                        .map(|v| (v.name.clone(), AbiType::from_string(&v.r#type)))
                        .collect(),
                };

                tokens.push(cairo_entry.expand_decl());
                tokens.push(cairo_entry.expand_impl());
            }
            AbiEntry::Event(ev) => {
            }
            _ => (),
        }
    }

    tokens.push(quote! {
        impl #contract_name {
            #(#functions)*
        }
    });

    let expanded = quote! {
        #(#tokens)*
    };

    expanded.into()
}
