use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Ident, LitStr, Token,
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
