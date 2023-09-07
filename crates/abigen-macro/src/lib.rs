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

    for entry in abi {
        match entry {
            AbiEntry::Struct(s) => {
                println!("{:?}", s);
            }
            AbiEntry::Function(f) => {
                println!("{:?}", f);
            }
            AbiEntry::Enum(e) => {
                println!("{:?}", e);
            }
            AbiEntry::Event(ev) => {
                println!("{:?}", ev);
            }
            _ => (),
        }
    }

    let expanded = quote! {
        #(#tokens)*
    };

    expanded.into()
}
