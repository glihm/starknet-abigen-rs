use starknet::core::types::contract::*;
use std::fs;
use syn::{
    parse::{Parse, ParseStream, Result}, Ident, LitStr, Token,
};

use starknet::{
    core::{chain_id, types::*},
    providers::{
        jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider, ProviderError,
        SequencerGatewayProvider,
    },
};
use url::Url;

#[derive(Clone, Debug)]
pub(crate) struct ContractFetch {
    // pub name: Ident,
    // pub abi: Vec<AbiEntry>,
}

impl Parse for ContractFetch {
    fn parse(input: ParseStream) -> Result<Self> {

        // let name = input.parse::<Ident>()?;
        // input.parse::<Token![,]>()?;

        // let address = input.parse::<LitStr>()?;
        // input.parse::<Token![,]>()?;

        Ok(ContractFetch {
        })

        // let rpc_url = input.parse::<LitStr>()?;
        // let rpc_url = Url::parse(&rpc_url.value())
        //     .map_err(|e|
        //              syn::Error::new(rpc_url.span(), format!("RPC URL error: {}", e)))?;

        // let provider = AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url)));

        // let address = FieldElement::from_hex_be(&address.value())
        //     .map_err(|e|
        //              syn::Error::new(address.span(), format!("Address convertion error: {}", e)))?;

        // let class = provider
        //     .get_class_at(BlockId::Tag(BlockTag::Latest), address)
        //     .await
        //     .map_err(|e|
        //              syn::Error::new(address.span(), format!("Fetch ABI error: {}", e)))?;

        // let class_json = serde_json::to_value(class)
        //     .map_err(|e|
        //              syn::Error::new(address.span(), format!("Parse ABI error: {}", e)))?;;

    }
}
