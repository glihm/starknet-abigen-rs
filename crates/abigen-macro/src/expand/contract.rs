//! Expands the contract first implementation with
//! default configuration for provider and account, if any.
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use syn::{
    Ident,
};

pub struct CairoContract;

impl CairoContract {
    pub fn expand(contract_name: Ident) -> TokenStream2 {
        quote! {
            #[derive(Debug)]
            pub struct #contract_name<'a>
            {
                pub address: starknet::core::types::FieldElement,
                pub provider: &'a starknet::providers::AnyProvider,
                pub account : std::option::Option<&'a starknet::accounts::SingleOwnerAccount<&'a starknet::providers::AnyProvider, starknet::signers::LocalWallet>>,
            }

            // TODO: Perhaps better than anyhow, a custom error?
            impl<'a> #contract_name<'a> {
                pub fn new(
                    address: starknet::core::types::FieldElement,
                    provider: &'a starknet::providers::AnyProvider,
                ) -> Self {
                    Self {
                        address,
                        provider,
                        account: None,
                    }
                }

                pub fn with_account(mut self, account: &'a starknet::accounts::SingleOwnerAccount<&'a starknet::providers::AnyProvider, starknet::signers::LocalWallet>,
                ) -> Self {
                    self.account = Some(account);
                    self
                }
            }
        }
    }
}
