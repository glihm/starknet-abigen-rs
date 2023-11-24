//! # Contract types expansion
//!
//! This module contains the auto-generated types
//! for the contract for which the bindings are requested.
//!
//! A contracts has two structs being auto-generated:
//!
//! * <contract_name>Reader - Which is a struct for readonly operations like `FunctionCall`.
//!   A reader can be initialized from a provider and the contract address.
//! * <contract_name> - A struct which is used to send transaction like `Invoke`.
//!   This struct must be initialized with an account and the contract address.
//!   A reader can also be initialized from this struct to avoid passing both a reader
//!   and the contract as arguments to functions.
//!
//! ## Examples
//!
//! ```ignore (pseudo-code)
//! let reader = ContractNameReader::new(contract_address, provider);
//!
//! let contract = ContractName::new(contract_address, account);
//!
//! let reader_from_contract = contract.reader();
//! ```
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

                pub fn provider(&self) -> &A::Provider {
                    self.account.provider()
                }

                // Code adapted from Starkli: https://github.com/xJonathanLEI/starkli
                pub async fn get_tx_status(&self, transaction_hash: FieldElement) -> starknet_abigen_parser::call::TransactionStatus {
                    use starknet_abigen_parser::call::TransactionStatus;
                    use starknet::{
                        core::types::{ExecutionResult, FieldElement, StarknetError},
                        providers::{MaybeUnknownErrorCode, Provider, ProviderError, StarknetErrorWithMessage},
                    };

                    match self.provider().get_transaction_receipt(transaction_hash).await {
                        Ok(receipt) => match receipt.execution_result() {
                            ExecutionResult::Succeeded => {
                                TransactionStatus::Succeeded
                            }
                            ExecutionResult::Reverted { reason } => {
                                TransactionStatus::Reverted(format!("reverted: {}", reason))
                            }
                        },
                        Err(ProviderError::StarknetError(StarknetErrorWithMessage {
                            code: MaybeUnknownErrorCode::Known(StarknetError::TransactionHashNotFound),
                            ..
                        })) => {
                            TransactionStatus::Pending
                        }
                        // Some nodes are still serving error code `25` for tx hash not found. This is
                        // technically a bug on the node's side, but we maximize compatibility here by also
                        // accepting it.
                        Err(ProviderError::StarknetError(StarknetErrorWithMessage {
                            code: MaybeUnknownErrorCode::Known(StarknetError::InvalidTransactionIndex),
                            ..
                        })) => {
                            TransactionStatus::Pending
                        }
                        Err(err) => TransactionStatus::Error(format!("error: {err}"))
                    }
                }
            }

            #[derive(Debug)]
            pub struct #reader<P: starknet::providers::Provider + Sync> {
                pub address: starknet::core::types::FieldElement,
                pub provider: P,
            }

            impl<P: starknet::providers::Provider + Sync> #reader<P> {
                pub fn new(
                    address: starknet::core::types::FieldElement,
                    provider: P,
                ) -> Self {
                    Self { address, provider }
                }

                pub fn provider(&self) -> &P {
                    &self.provider
                }
            }
        };

        q
    }
}
