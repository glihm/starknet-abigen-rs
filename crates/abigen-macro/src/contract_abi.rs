use starknet::core::types::contract::*;
use std::fs;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Ident, LitStr, Token,
};

#[derive(Clone, Debug)]
pub(crate) struct ContractAbi {
    pub name: Ident,
    pub abi: Vec<AbiEntry>,
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
            Err(e) => {
                println!("Error loading the input as ABI: {:?}\nTry to load a JSON file",
                         e);

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
