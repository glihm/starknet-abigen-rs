//! # Functions types expansion (cairo 0).
//!
use super::{
    utils::{str_to_ident, str_to_type},
    ExpandableFunction,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use starknet::core::types::contract::StateMutability;

use starknet_abigen_parser::{
    abi_types::{AbiType, AbiTypeAny},
    CairoFunctionLegacy,
};

fn get_func_inputs(inputs: &[(String, AbiTypeAny)]) -> Vec<TokenStream2> {
    let mut out: Vec<TokenStream2> = vec![];

    for (name, abi_type) in inputs {
        let name = str_to_ident(name);
        let ty = str_to_type(&abi_type.to_rust_type_legacy_check(true));
        // We can pass a reference here as serialize always takes a reference.
        out.push(quote!(#name:&#ty));
    }

    out
}

fn get_func_outputs(outputs: &[AbiTypeAny]) -> Vec<TokenStream2> {
    let mut out: Vec<TokenStream2> = vec![];

    for abi_type in outputs {
        let ty = str_to_type(&abi_type.to_rust_type_legacy_check(true));
        out.push(quote!(#ty));
    }

    out
}

impl ExpandableFunction for CairoFunctionLegacy {
    fn expand_decl(&self) -> TokenStream2 {
        quote!()
    }

    fn expand_impl(&self) -> TokenStream2 {
        let _decl = self.expand_decl();
        let func_name = &self.name;
        let func_name_ident = str_to_ident(&self.name);

        let mut serializations: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.inputs {
            let name = str_to_ident(name);
            let ty = str_to_type(&abi_type.to_rust_type_path_legacy_check(true));

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
        } else {
            let otys = get_func_outputs(&self.outputs);
            quote!((#(#otys),*))
        };

        let inputs = get_func_inputs(&self.inputs);
        let func_name_call = str_to_ident(&format!("{}_getcall", self.name));
        let type_param = if is_for_reader {
            str_to_type("P")
        } else {
            str_to_type("A::Provider")
        };

        match &self.state_mutability {
            StateMutability::View => quote! {
                pub fn #func_name_ident(
                    &self,
                    #(#inputs),*
                ) -> starknet_abigen_parser::call::FCall<#type_param, #out_type> {
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
