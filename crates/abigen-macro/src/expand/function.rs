use crate::expand::utils::{str_to_ident, str_to_type};
use crate::Expandable;
use cairo_type_parser::abi_type::AbiType;
use cairo_type_parser::CairoFunction;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use starknet::core::types::contract::StateMutability;

impl Expandable for CairoFunction {
    fn expand_decl(&self) -> TokenStream2 {
        let func_name = str_to_ident(&self.name.get_type_name_only());

        let mut inputs: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.inputs {
            let name = str_to_ident(&name);
            let ty = str_to_type(&abi_type.to_rust_type());
            inputs.push(quote!(#name:#ty));
        }

        let output = match self.state_mutability {
            StateMutability::View => match &self.output {
                // TODO: perhaps anyhow is not the best here, custom error may be better?
                Some(o) => {
                    let oty = str_to_type(&o.to_rust_type());
                    quote!(-> cairo_types::Result<#oty>)
                }
                None => quote!(),
            },
            StateMutability::External => {
                // Only the TX hash is returned on success.
                // TODO: go away from anyhow? Should it
                // be something like cairo_contracts::Result?
                quote!(-> anyhow::Result<starknet::core::types::FieldElement>)
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
        let func_name = &self.name.get_type_name_only();

        let mut serializations: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.inputs {
            let name = str_to_ident(&name);
            let ty = str_to_type(&abi_type.to_rust_item_path());
            serializations.push(quote! {
                calldata.extend(#ty::serialize(&#name));
            });
        }

        let out_res = match &self.output {
            Some(o) => {
                let out_item_path = str_to_type(&o.to_rust_item_path());
                match o {
                    // Tuples type used as rust type path must be surrounded
                    // by LT/GT.
                    AbiType::Tuple(_) => quote!(<#out_item_path>::deserialize(r, 0)),
                    _ => quote!(#out_item_path::deserialize(&r, 0)),
                }
            }
            None => quote!(),
        };

        match &self.state_mutability {
            StateMutability::View => quote! {
                #decl {
                    let mut calldata = vec![];
                    #(#serializations)*

                    let r = self.provider
                        .call(
                            starknet::core::types::FunctionCall {
                                contract_address: self.address,
                                entry_point_selector: starknet::macros::selector!(#func_name),
                                calldata,
                            },
                            BlockId::Tag(BlockTag::Pending),
                        )
                        .await.map_err(
                            |err|
                            cairo_types::Error::Deserialize(
                                format!("Deserialization error {:}", err)))?;

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
                #decl {
                    // TODO: I don't know how to easily store the SingleOwnerAccount
                    // and it's generic types without complexifiying the whole typing.
                    // So it's constructed at every call. There is surely a better approach.
                    let (account_address, signer) = match (self.account_address, &self.signer) {
                        (Some(a), Some(s)) => (a, s),
                        // TODO: better error handling here.
                        _ => return Err(anyhow::anyhow!("Account address and signer are required to send invoke transactions"))
                    };

                    let account = starknet::accounts::SingleOwnerAccount::new(
                        &self.provider,
                        signer,
                        account_address,
                        self.chain_id
                    );

                    let mut calldata = vec![];
                    #(#serializations)*

                    let calls = vec![starknet::accounts::Call {
                        to: self.address,
                        selector: starknet::macros::selector!(#func_name),
                        calldata,
                    }];

                    let execution = account.execute(calls).fee_estimate_multiplier(2f64);
                    // TODO: we can have manual fee here, or it can also be estimate only.
                    let max_fee = execution.estimate_fee().await?.overall_fee.into();

                    let invoke_tx = execution
                        .max_fee(max_fee)
                        .send().await?.transaction_hash;

                    // TODO: add an option to watch and wait for the tx to have a receipt?
                    Ok(invoke_tx)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Expandable;
    use cairo_type_parser::{abi_type::AbiType, CairoFunction};
    use proc_macro2::TokenStream;
    use quote::quote;
    use starknet::core::types::contract::StateMutability;

    #[test]
    fn test_decl_basic() {
        let cf = CairoFunction {
            name: AbiType::Basic("my_func".to_string()),
            state_mutability: StateMutability::View,
            inputs: vec![
                (
                    "v1".to_string(),
                    AbiType::Basic("core::felt252".to_string()),
                ),
                (
                    "v2".to_string(),
                    AbiType::Basic("core::felt252".to_string()),
                ),
            ],
            output: Some(AbiType::Basic("core::felt252".to_string())),
        };
        let te1 = cf.expand_decl();
        let tef1: TokenStream = quote!(
            pub async fn my_func(&self, v1: starknet::core::types::FieldElement, v2: starknet::core::types::FieldElement) -> anyhow::Result<starknet::core::types::FieldElement>
        );

        assert_eq!(te1.to_string(), tef1.to_string());
    }

    #[test]
    fn test_impl_basic() {
        let cf = CairoFunction {
            name: AbiType::Basic("my_func".to_string()),
            state_mutability: StateMutability::View,
            inputs: vec![
                (
                    "v1".to_string(),
                    AbiType::Basic("core::felt252".to_string()),
                ),
                (
                    "v2".to_string(),
                    AbiType::Basic("core::felt252".to_string()),
                ),
            ],
            output: Some(AbiType::Basic("core::felt252".to_string())),
        };
        let te1 = cf.expand_impl();

        #[rustfmt::skip]
        let tef1: TokenStream = quote!(
            pub async fn my_func(
                &self,
                v1: starknet::core::types::FieldElement,
                v2: starknet::core::types::FieldElement
            ) -> anyhow::Result<starknet::core::types::FieldElement> {
                let mut calldata = vec![];
                calldata.extend(starknet::core::types::FieldElement::serialize(&v1));
                calldata.extend(starknet::core::types::FieldElement::serialize(&v2));

                let r = self.provider
                    .call(
                        starknet::core::types::FunctionCall {
                            contract_address: self.address,
                            entry_point_selector: starknet::macros::selector!("my_func"),
                            calldata,
                        },
                        BlockId::Tag(BlockTag::Pending),
                    )
                    .await?;

                starknet::core::types::FieldElement::deserialize(r, 0)
            }
        );

        assert_eq!(te1.to_string(), tef1.to_string());
    }
}
