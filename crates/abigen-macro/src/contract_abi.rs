//! Defines the arguments of the `abigen` macro.
//!
//! `ContractAbi` is expected to the argument
//! passed to the macro. We should then parse the
//! token stream to ensure the arguments are correct.
//!
//! At this moment, the macro supports two fashions:
//!
//! Loading from a file.
//! ```
//! abigen!(ContractName, "path/to/abi.json"
//! ```
//!
//! Loading from a literal string ABI.
//! ```
//! abigen!(ContractName, r#"
//!    [{ .... }]
//! #");
//! ```
use starknet::core::types::contract::*;
use std::fs;
use syn::{
    parse::{Parse, ParseStream, Result},
    Ident, LitStr, Token,
};

#[derive(Clone, Debug)]
pub(crate) struct ContractAbi {
    pub name: Ident,
    pub abi: Vec<AbiEntry>,
}

impl Parse for ContractAbi {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;

        let content = input.parse::<LitStr>()?;

        // abi (from ether-rs crate).
        // Due to limitation with the proc-macro Span API, we
        // can't currently get a path the the file where we were called from;
        // therefore, the path will always be rooted on the cargo manifest
        // directory. Eventually we can use the `Span::source_file` API to
        // have a better experience.
        match serde_json::from_str(&content.value()) {
            Ok(abi_json) => Ok(ContractAbi {
                name,
                abi: abi_json,
            }),
            Err(e) => {
                // TODO: check how we can notify better the user in case of error.
                // Or do we want 2 macros to easy debugging / better UX on error?
                println!(
                    "Error loading the input as ABI: {:?}\nTry to load a JSON file",
                    e
                );

                let path = content;
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
