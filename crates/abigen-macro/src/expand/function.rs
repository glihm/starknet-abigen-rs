use crate::expand::utils::{str_to_ident, str_to_type};
use crate::Expandable;
use cairo_type_parser::{CairoFunction, CairoFunctionStateMutability};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use starknet::core::utils::get_selector_from_name;

impl Expandable for CairoFunction {
    fn expand_decl(&self) -> TokenStream2 {
        let func_name = str_to_ident(&self.name.get_type_name_only());

        let mut inputs: Vec<TokenStream2> = vec![];
        let mut outputs: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.inputs {
            let name = str_to_ident(&name);
            let ty = str_to_type(&abi_type.to_rust_type());
            inputs.push(quote!(#name:#ty));
        }

        for abi_type in &self.outputs {
            let ty = str_to_type(&abi_type.to_rust_type());
            outputs.push(quote!(#ty));
        }

        let final_outputs = if outputs.len() > 0 {
            quote! {
                -> anyhow::Result<#(#outputs),*>
            }
        } else {
            quote!()
        };

        quote! {
            pub async fn #func_name(
                #(#inputs),*
            ) #final_outputs
        }
    }

    fn expand_impl(&self) -> TokenStream2 {
        let decl = self.expand_decl();
        let func_name = &self.name.get_type_name_only();

        let mut serializations: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.inputs {
            let name = str_to_ident(&name);
            let ty = str_to_type(&abi_type.to_rust_item_path(true));
            serializations.push(quote! {
                calldata.extend(#ty::serialize(&#name));
            });
        }

        match &self.state_mutability {
            CairoFunctionStateMutability::View => quote! {
                #decl {
                    let mut calldata = vec![];
                    #(#serializations)*

                    self.provider
                        .call(
                            FunctionCall {
                                contract_address: self.address,
                                entry_point_selector: starknet::macros::selector!(#func_name),
                                calldata,
                            },
                            BlockId::Tag(BlockTag::Pending),
                        )
                        .await?
                }
            },
            CairoFunctionStateMutability::External => quote! {},
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Expandable;
    use cairo_type_parser::{abi_type::AbiType, CairoFunction, CairoFunctionStateMutability};
    use proc_macro2::TokenStream;
    use quote::quote;

    #[test]
    fn test_decl_basic() {
        let cf = CairoFunction {
            name: AbiType::Basic("my_func".to_string()),
            state_mutability: CairoFunctionStateMutability::View,
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
            outputs: vec![AbiType::Basic("core::felt252".to_string())],
        };
        let te1 = cf.expand_decl();
        let tef1: TokenStream = quote!(
            pub async fn my_func(v1: starknet::core::types::FieldElement, v2: starknet::core::types::FieldElement) -> anyhow::Result<starknet::core::types::FieldElement>
        );

        assert_eq!(te1.to_string(), tef1.to_string());
    }

    #[test]
    fn test_impl_basic() {
        let cf = CairoFunction {
            name: AbiType::Basic("my_func".to_string()),
            state_mutability: CairoFunctionStateMutability::View,
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
            outputs: vec![AbiType::Basic("core::felt252".to_string())],
        };
        let te1 = cf.expand_impl();

        #[rustfmt::skip]
        let tef1: TokenStream = quote!(
            pub async fn my_func(
                v1: starknet::core::types::FieldElement,
                v2: starknet::core::types::FieldElement
            ) -> anyhow::Result<starknet::core::types::FieldElement> {
                let mut calldata = vec![];
                calldata.extend(starknet::core::types::FieldElement::serialize(&v1));
                calldata.extend(starknet::core::types::FieldElement::serialize(&v2));

                self.provider
                    .call(
                        FunctionCall {
                            contract_address: self.address,
                            entry_point_selector: starknet::macros::selector!("my_func"),
                            calldata,
                        },
                        BlockId::Tag(BlockTag::Pending),
                    )
                    .await?
            }
        );

        assert_eq!(te1.to_string(), tef1.to_string());
    }
}
