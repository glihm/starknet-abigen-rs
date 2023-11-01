use super::{
    utils::{str_to_ident, str_to_type},
    Expandable,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use starknet::core::types::contract::StateMutability;
use starknet_abigen_parser::{
    abi_types::{AbiType, AbiTypeAny},
    CairoFunction,
};

impl Expandable for CairoFunction {
    fn expand_decl(&self) -> TokenStream2 {
        let func_name = str_to_ident(&self.name);

        let mut inputs: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.inputs {
            let name = str_to_ident(name);
            let ty = str_to_type(&abi_type.to_rust_type());
            // We can pass a reference here as serialize always takes a reference.
            inputs.push(quote!(#name:&#ty));
        }

        let output = match self.state_mutability {
            StateMutability::View => match &self.output {
                Some(o) => {
                    let oty = str_to_type(&o.to_rust_type());
                    quote!(-> starknet_abigen_parser::cairo_types::Result<#oty>)
                }
                None => {
                    quote!(-> starknet_abigen_parser::cairo_types::Result<()>)
                }
            },
            StateMutability::External => {
                quote!(-> Result<starknet::core::types::InvokeTransactionResult,
                       starknet::accounts::AccountError<A::SignError, <A::Provider as starknet::providers::Provider>::Error>>
                )
            }
        };

        quote! {
            pub async fn #func_name(
                &self,
                #(#inputs),*
            ) #output
        }
    }

    fn expand_impl(&self) -> TokenStream2 {
        let decl = self.expand_decl();
        let func_name = &self.name;

        let mut serializations: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.inputs {
            let name = str_to_ident(name);
            let ty = str_to_type(&abi_type.to_rust_type_path());

            let ser = match abi_type {
                AbiTypeAny::Tuple(_) => quote! {
                    __calldata.extend(<#ty>::serialize(#name));
                },
                _ => quote!(__calldata.extend(#ty::serialize(#name));),
            };
            serializations.push(ser);
        }

        let out_res = match &self.output {
            Some(o) => {
                let out_type_path = str_to_type(&o.to_rust_type_path());
                match o {
                    // Tuples type used as rust type path must be surrounded
                    // by LT/GT.
                    AbiTypeAny::Tuple(_) => quote!(<#out_type_path>::deserialize(&r, 0)),
                    _ => quote!(#out_type_path::deserialize(&r, 0)),
                }
            }
            None => quote!(Ok(())),
        };

        match &self.state_mutability {
            StateMutability::View => quote! {
                #[allow(clippy::ptr_arg)]
                #decl {
                    use starknet_abigen_parser::CairoType;
                    use starknet::core::types::{BlockId, BlockTag};

                    let mut __calldata = vec![];
                    #(#serializations)*

                    let r = self.provider
                        .call(
                            starknet::core::types::FunctionCall {
                                contract_address: self.address,
                                entry_point_selector: starknet::macros::selector!(#func_name),
                                calldata: __calldata,
                            },
                            self.call_block_id,
                        )
                        .await.map_err(
                            |err|
                            starknet_abigen_parser::cairo_types::Error::Deserialize(
                                format!("Deserialization error {}", err)))?;

                    #out_res
                }
            },
            StateMutability::External => quote! {
                // TODO: How can we add Fee configuration + estimate fee out of the box.
                // maybe two methods are generated, one for actually running, the other
                // for estimate the fees.
                // Or, we can add a config struct as the last argument? Or directly
                // at the initialization of the contract, we can give a config for
                // fees (manual, estimated + scale factor).
                // The estimate only may be done at the function level, to avoid
                // altering the contract instance itself and hence races.
                #[allow(clippy::ptr_arg)]
                #decl {
                    use starknet_abigen_parser::CairoType;
                    use starknet::accounts::Account;

                    let mut __calldata = vec![];
                    #(#serializations)*

                    let calls = vec![starknet::accounts::Call {
                        to: self.address,
                        selector: starknet::macros::selector!(#func_name),
                        calldata: __calldata,
                    }];

                    // TODO: add a way for fee estimation and max fee to be parametrizable.
                    self.account.execute(calls).send().await
                }
            },
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
