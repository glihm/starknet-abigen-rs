mod expand;
use expand::contract::CairoContract;

mod contract_abi;
use contract_abi::ContractAbi;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use starknet::core::types::contract::*;
use std::fs;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Ident, LitStr, Token,
};
use std::collections::HashMap;

use cairo_type_parser::CairoStruct;
use cairo_type_parser::abi_types::{AbiType, AbiTypeAny};
// use cairo_types::ty::{CAIRO_BASIC_ENUMS, CAIRO_BASIC_STRUCTS};

trait Expandable {
    fn expand_decl(&self) -> TokenStream2;
    fn expand_impl(&self) -> TokenStream2;
}

#[proc_macro]
pub fn abigen(input: TokenStream) -> TokenStream {
    let contract_abi = parse_macro_input!(input as ContractAbi);
    let contract_name = contract_abi.name;
    let abi = contract_abi.abi;

    let mut tokens: Vec<TokenStream2> = vec![];

    tokens.push(CairoContract::expand(contract_name));

    let mut structs: HashMap<String, CairoStruct> = HashMap::new();

    for entry in abi {
        match entry {
            AbiEntry::Struct(s) => {
                let abi_type = AbiTypeAny::from_string(&s.name);
                let cs = CairoStruct::new(&s.name, &s.members);

                if let Some(ref mut existing_cs) = structs.get_mut(&cs.get_name()) {
                    cs.compare_generic_types(existing_cs);
                } else {
                    structs.insert(cs.get_name(), cs.clone());
                }
            }
            // Enum
            // Events
            _ => continue
        }
    }

    // For the functions, we must take any existing enum or struct.
    // as we will have the correct rust type for generics.
    // But as we need filtered structs and enum, this must be done
    // in a second loop when all structs/enums are parsed.
    for entry in abi {
        match entry {
            // Functions only.
            _ => continue
        }
    }

    // Only expand unique structs.
    for (_, cs) in structs {
        tokens.push(cs.expand_decl());
        tokens.push(cs.expand_impl());
    }

    // Enums.

    // Functions.

    let expanded = quote! {
        #(#tokens)*
    };

    expanded.into()
}
