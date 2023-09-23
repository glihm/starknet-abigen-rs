mod expand;
use expand::contract::CairoContract;

mod contract_abi;
use contract_abi::ContractAbi;

mod contract_fetch;
use contract_fetch::ContractFetch;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use starknet::core::types::contract::*;

use syn::{
    parse_macro_input,
};
use std::collections::HashMap;

use cairo_type_parser::{CairoStruct, CairoEnum, CairoFunction};
use cairo_type_parser::abi_types::{AbiType, AbiTypeAny};
use cairo_types::{CAIRO_BASIC_ENUMS, CAIRO_BASIC_STRUCTS};

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

    tokens.push(CairoContract::expand(contract_name.clone()));

    let mut structs: HashMap<String, CairoStruct> = HashMap::new();
    let mut enums: HashMap<String, CairoEnum> = HashMap::new();

    for entry in &abi {
        match entry {
            AbiEntry::Struct(s) => {
                let cs = CairoStruct::new(&s.name, &s.members);

                if CAIRO_BASIC_STRUCTS.contains(&cs.get_name().as_str()) {
                    continue;
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
                    continue;
                }

                if let Some(ref mut existing_ce) = enums.get_mut(&ce.get_name()) {
                    ce.compare_generic_types(existing_ce);
                } else {
                    enums.insert(ce.get_name(), ce.clone());
                }
            }
            AbiEntry::Event(_ev) => {
                // TODO events.
            }
            // Events
            _ => continue
        }
    }

    // For the functions, we must take any existing enum or struct.
    // as we will have the correct rust type for generics.
    // But as we need filtered structs and enum, this must be done
    // in a second loop when all structs/enums are parsed.
    let mut functions = vec![];

    for entry in &abi {
        match entry {
            AbiEntry::Function(f) => {

                // Functions cannot be generic when they are entry point.
                // From this statement, we can safely assume that any function name is unique,
                // and we only have to care about arguments that may contain generic.
                let cf = CairoFunction::new(
                    &f.name,
                    f.state_mutability.clone(),
                    &f.inputs,
                    &f.outputs,
                );

                functions.push(cf.expand_impl());
            }
            _ => continue
        }
    }

    // Expand only once structs and enums taking generics in account.
    for (_, cs) in structs {
        tokens.push(cs.expand_decl());
        tokens.push(cs.expand_impl());
    }

    for (_, ce) in enums {
        tokens.push(ce.expand_decl());
        tokens.push(ce.expand_impl());
    }

    tokens.push(quote! {
        impl<'a> #contract_name<'a>
        {
            #(#functions)*
        }
    });

    let expanded = quote! {
        #(#tokens)*
    };

    expanded.into()
}
