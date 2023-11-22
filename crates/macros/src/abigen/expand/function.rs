//! # Functions types expansion
//!
//! This module contains the auto-generated types
//! for the functions of a contract for which the bindings are requested.
//!
//! Starknet has two types of functions:
//!
//! * `Views` - Which are also named `FunctionCall` that don't modifying the state. Readonly operations.
//! * `Externals` - Where a transaction is involved and can alter the state. Write operations.
//!
//! For each of these functions, there is a struct that is dedicated for each function of the contract,
//! based on it's state mutability found in the ABI itself.
//!
//! * `FCall` - Struct for readonly functions.
//! * `FInvoke` - Struct for transaction based functions.
//!
//! ## Examples
//!
//! ```ignore (pseudo-code)
//! // TODO
//! ```
use super::{
    utils::{str_to_ident, str_to_type},
    Expandable,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use starknet::core::types::contract::StateMutability;

use starknet_abigen_parser::{abi_types::AbiTypeAny, CairoFunction};

fn get_func_inputs(inputs: &[(String, AbiTypeAny)], is_legacy: bool) -> Vec<TokenStream2> {
    let mut out: Vec<TokenStream2> = vec![];

    for (name, abi_type) in inputs {
        let name = str_to_ident(name);
        let ty = str_to_type(&abi_type.to_rust_type_legacy_check(is_legacy));
        // We can pass a reference here as serialize always takes a reference.
        out.push(quote!(#name:&#ty));
    }

    out
}

fn get_func_outputs(outputs: &[AbiTypeAny], is_legacy: bool) -> Vec<TokenStream2> {
    let mut out: Vec<TokenStream2> = vec![];

    for abi_type in outputs {
        let ty = str_to_type(&abi_type.to_rust_type_legacy_check(is_legacy));
        out.push(quote!(#ty));
    }

    out
}

impl Expandable for CairoFunction {
    fn expand_decl(&self, _is_legacy: bool) -> TokenStream2 {
        quote!()
    }

    fn expand_impl(&self, is_legacy: bool) -> TokenStream2 {
        let _decl = self.expand_decl(is_legacy);
        let func_name = &self.name;
        let func_name_ident = str_to_ident(&self.name);

        let mut serializations: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.inputs {
            let name = str_to_ident(name);
            let ty = str_to_type(&abi_type.to_rust_type_path_legacy_check(is_legacy));

            let ser = match abi_type {
                AbiTypeAny::Tuple(_) => quote! {
                    __calldata.extend(<#ty>::serialize(#name));
                },
                _ => quote!(__calldata.extend(#ty::serialize(#name));),
            };
            serializations.push(ser);
        }

        let out_type = if self.outputs.is_empty() {
            quote!(())
        } else if is_legacy {
            // Cairo 0 always returns a tuple of values, if any.
            let otys = get_func_outputs(&self.outputs, is_legacy);
            quote!((#(#otys),*))
        } else {
            // We consider only one type for Cairo 1, if any.
            // The outputs is a list for historical reason from Cairo 0
            // were tuples or output were used as returned values.
            let out_type = str_to_type(&self.outputs[0].to_rust_type_legacy_check(is_legacy));
            quote!(#out_type)
        };

        let inputs = get_func_inputs(&self.inputs, is_legacy);
        let func_name_call = str_to_ident(&format!("{}_getcall", self.name));

        match &self.state_mutability {
            StateMutability::View => quote! {
                #[allow(clippy::ptr_arg)]
                #[allow(clippy::too_many_arguments)]
                pub fn #func_name_ident(
                    &self,
                    #(#inputs),*
                ) -> starknet_abigen_parser::call::FCall<'p, P, #out_type> {
                    use starknet_abigen_parser::CairoType;

                    let mut __calldata = vec![];
                    #(#serializations)*

                    let __call = starknet::core::types::FunctionCall {
                        contract_address: self.address,
                        entry_point_selector: starknet::macros::selector!(#func_name),
                        calldata: __calldata,
                    };

                    starknet_abigen_parser::call::FCall::new(
                        __call,
                        self.provider,
                    )
                }
            },
            StateMutability::External => {
                // For now, Execution can't return the list of calls.
                // This would be helpful to easily access the calls
                // without having to add `_getcall()` method.
                // If starknet-rs provides a way to get the calls,
                // we can remove `_getcall()` method.
                //
                // TODO: if it's possible to do it with lifetime,
                // this can be try in an issue.
                quote! {
                    #[allow(clippy::ptr_arg)]
                    #[allow(clippy::too_many_arguments)]
                    pub fn #func_name_call(
                        &self,
                        #(#inputs),*
                    ) -> starknet::accounts::Call {
                        use starknet_abigen_parser::CairoType;

                        let mut __calldata = vec![];
                        #(#serializations)*

                        starknet::accounts::Call {
                            to: self.address,
                            selector: starknet::macros::selector!(#func_name),
                            calldata: __calldata,
                        }
                    }

                    #[allow(clippy::ptr_arg)]
                    pub fn #func_name_ident(
                        &self,
                        #(#inputs),*
                    ) -> starknet::accounts::Execution<A> {
                        use starknet_abigen_parser::CairoType;

                        let mut __calldata = vec![];
                        #(#serializations)*

                        let __call = starknet::accounts::Call {
                            to: self.address,
                            selector: starknet::macros::selector!(#func_name),
                            calldata: __calldata,
                        };

                        self.account.execute(vec![__call])
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::abigen::Expandable;
    use proc_macro2::TokenStream as TokenStream2;
    use quote::quote;
    use starknet::core::types::contract::StateMutability;
    use starknet_abigen_parser::{abi_types::AbiTypeAny, CairoFunction};

    #[test]
    fn test_decl_basic() {
        let cf = CairoFunction {
            name: "my_func".to_string(),
            state_mutability: StateMutability::View,
            inputs: vec![
                ("v1".to_string(), AbiTypeAny::Basic("core::felt252".into())),
                ("v2".to_string(), AbiTypeAny::Basic("core::felt252".into())),
            ],
            output: Some(AbiTypeAny::Basic("core::felt252".into())),
        };
        let te1 = cf.expand_decl();
        let tef1: TokenStream2 = quote!(
            pub async fn my_func(&self, v1: &starknet::core::types::FieldElement, v2: &starknet::core::types::FieldElement) -> starknet_abigen_parser::cairo_types::Result<starknet::core::types::FieldElement>
        );

        assert_eq!(te1.to_string(), tef1.to_string());
    }

    #[test]
    fn test_impl_basic() {
        let cf = CairoFunction {
            name: "my_func".to_string(),
            state_mutability: StateMutability::View,
            inputs: vec![
                ("v1".to_string(), AbiTypeAny::Basic("core::felt252".into())),
                ("v2".to_string(), AbiTypeAny::Basic("core::felt252".into())),
            ],
            output: Some(AbiTypeAny::Basic("core::felt252".into())),
        };
        let te1 = cf.expand_impl();

        #[rustfmt::skip]
        let tef1: TokenStream2 = quote!(
            #[allow(clippy::ptr_arg)]
            pub async fn my_func(
                &self,
                v1: &starknet::core::types::FieldElement,
                v2: &starknet::core::types::FieldElement
            ) -> starknet_abigen_parser::cairo_types::Result<starknet::core::types::FieldElement> {
                use starknet_abigen_parser::CairoType;
                use starknet::core::types::{BlockId, BlockTag};

                let mut __calldata = vec![];
                __calldata.extend(starknet::core::types::FieldElement::serialize(v1));
                __calldata.extend(starknet::core::types::FieldElement::serialize(v2));

                let r = self.provider
                    .call(
                        starknet::core::types::FunctionCall {
                            contract_address: self.address,
                            entry_point_selector: starknet::macros::selector!("my_func"),
                            calldata: __calldata,
                        },
                        self.call_block_id,
                    )
                    .await.map_err(|err| starknet_abigen_parser::cairo_types::Error::Deserialize(format!("Deserialization error {}" , err)))?;

                starknet::core::types::FieldElement::deserialize(&r, 0)
            }
        );

        assert_eq!(te1.to_string(), tef1.to_string());
    }
}
