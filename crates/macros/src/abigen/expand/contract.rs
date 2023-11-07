//! Expands the contract first implementation with
//! default configuration for provider and account, if any.
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use super::utils;

pub struct CairoContract;

impl CairoContract {
    pub fn expand(contract_name: Ident) -> TokenStream2 {
        let reader = utils::str_to_ident(format!("{}Reader", contract_name).as_str());
        let q = quote! {

            #[derive(Debug)]
            pub struct #contract_name<A: starknet::accounts::ConnectedAccount + Sync> {
                pub address: starknet::core::types::FieldElement,
                pub account: A,
            }

            impl<A: starknet::accounts::ConnectedAccount + Sync> #contract_name<A> {
                pub fn new(address: starknet::core::types::FieldElement, account: A) -> Self {
                    Self { address, account }
                }

                pub fn reader(&self) -> #reader<A::Provider> {
                    #reader::new(self.address, self.account.provider())
                }
            }

            #[derive(Debug)]
            pub struct #reader<'a, P: starknet::providers::Provider + Sync> {
                pub address: starknet::core::types::FieldElement,
                pub provider: &'a P,
                call_block_id: starknet::core::types::BlockId,
            }

            impl<'a, P: starknet::providers::Provider + Sync> #reader<'a, P> {
                pub fn new(
                    address: starknet::core::types::FieldElement,
                    provider: &'a P,
                ) -> Self {
                    let call_block_id = starknet::core::types::BlockId::Tag(starknet::core::types::BlockTag::Pending);
                    Self { address, provider, call_block_id }
                }

                pub fn set_call_block_id(&mut self, block_id: starknet::core::types::BlockId) {
                    self.call_block_id = block_id;
                }

                pub fn get_call_block_id(&self) -> starknet::core::types::BlockId {
                    self.call_block_id
                }

                // TODO: String is not the ideal, but we need to export an enum somewhere for that.
                pub async fn get_tx_status(&self, transaction_hash: FieldElement) -> String {
                    use starknet::{
                        core::types::{ExecutionResult, FieldElement, StarknetError},
                        providers::{MaybeUnknownErrorCode, Provider, ProviderError, StarknetErrorWithMessage},
                    };

                    match self.provider.get_transaction_receipt(transaction_hash).await {
                        Ok(receipt) => match receipt.execution_result() {
                            ExecutionResult::Succeeded => {
                                "ok".to_string()
                            }
                            ExecutionResult::Reverted { reason } => {
                                format!("reverted: {}", reason)
                            }
                        },
                        Err(ProviderError::StarknetError(StarknetErrorWithMessage {
                            code: MaybeUnknownErrorCode::Known(StarknetError::TransactionHashNotFound),
                            ..
                        })) => {
                            "pending".to_string()
                        }
                        // Some nodes are still serving error code `25` for tx hash not found. This is
                        // technically a bug on the node's side, but we maximize compatibility here by also
                        // accepting it.
                        Err(ProviderError::StarknetError(StarknetErrorWithMessage {
                            code: MaybeUnknownErrorCode::Known(StarknetError::InvalidTransactionIndex),
                            ..
                        })) => {
                            "pending".to_string()
                        }
                        Err(err) => format!("error: {err}")
                    }
                }

            }
        };

        q
    }
}
