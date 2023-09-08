use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use serde_json;
use starknet::core::types::contract::*;
use std::fs;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Ident, LitStr, Token,
};

use cairo_type_parser::{abi_type::AbiType, CairoEnum};
use cairo_type_parser::{CairoFunction, CairoStruct};
use cairo_types::ty::{CAIRO_BASIC_ENUMS, CAIRO_BASIC_STRUCTS};

mod expand;

trait Expandable {
    fn expand_decl(&self) -> TokenStream2;
    fn expand_impl(&self) -> TokenStream2;
}

#[derive(Clone, Debug)]
pub(crate) struct ContractAbi {
    name: Ident,
    abi: Vec<AbiEntry>,
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
        let contents = input.parse::<LitStr>()?;
        match serde_json::from_str(&contents.value()) {
            Ok(abi_json) => Ok(ContractAbi {
                name,
                abi: abi_json,
            }),
            Err(_) => {
                let path = contents;
                match fs::read_to_string(path.value()) {
                    Ok(abi_str) => {
                        let abi_json = serde_json::from_str(&abi_str).map_err(|e| {
                            syn::Error::new(path.span(), format!("JSON error: {}", e))
                        })?;
                        Ok(ContractAbi {
                            name,
                            abi: abi_json,
                        })
                    }
                    Err(err) => Err(syn::Error::new(path.span(), format!("File error: {}", err))),
                }
            }
        }
    }
}

#[proc_macro]
pub fn abigen(input: TokenStream) -> TokenStream {
    let contract_abi = parse_macro_input!(input as ContractAbi);
    let contract_name = contract_abi.name;
    let abi = contract_abi.abi;

    let mut tokens: Vec<TokenStream2> = vec![];

    // TODO: fix imports. Do we want to import everything at the top
    // of the contract, and put the contract inside a module?

    tokens.push(quote! {
        #[derive(Debug)]
        pub struct #contract_name {
            pub address: starknet::core::types::FieldElement,
            provider: starknet::providers::AnyProvider,
            account_address: std::option::Option<starknet::core::types::FieldElement>,
            signer: std::option::Option<starknet::signers::LocalWallet>,
            chain_id: starknet::core::types::FieldElement,
        }

        // TODO: Perhaps better than anyhow, a custom error?
        // TODO: Make provider reference
        impl #contract_name {
            pub async fn new_caller(
                address: starknet::core::types::FieldElement,
                provider: starknet::providers::AnyProvider,
            ) -> anyhow::Result<#contract_name> {
                use starknet::providers::Provider;
                let chain_id = provider.chain_id().await?;

                Ok(#contract_name {
                    address,
                    provider,
                    account_address: None,
                    signer: None,
                    chain_id,
                })
            }

            pub async fn new_invoker(
                address: starknet::core::types::FieldElement,
                provider: starknet::providers::AnyProvider,
                account_address: starknet::core::types::FieldElement,
                signer: starknet::signers::LocalWallet,
            ) -> anyhow::Result<#contract_name> {
                use starknet::providers::Provider;
                let chain_id = provider.chain_id().await?;

                Ok(#contract_name {
                    address,
                    provider,
                    account_address: Some(account_address),
                    signer: Some(signer),
                    chain_id,
                })
            }
        }
    });

    let mut functions = vec![];

    for entry in abi {
        match entry {
            AbiEntry::Struct(s) => {
                let cairo_entry = CairoStruct {
                    name: AbiType::from_string(&s.name),
                    members: s
                        .members
                        .iter()
                        .map(|m| (m.name.clone(), AbiType::from_string(&m.r#type)))
                        .collect(),
                };

                if CAIRO_BASIC_STRUCTS.contains(&cairo_entry.name.get_type_name_only().as_str()) {
                    continue;
                }

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

                if CAIRO_BASIC_ENUMS.contains(&cairo_entry.name.get_type_name_only().as_str()) {
                    continue;
                }

                tokens.push(cairo_entry.expand_decl());
                tokens.push(cairo_entry.expand_impl());
            }
            AbiEntry::Event(_) => {}
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
