use anyhow::{anyhow, Result};
use clap::Parser;
use url::Url;

use starknet::core::types::contract::*;

use starknet::{
    core::{types::*},
    providers::{
        jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider,
    },
};

use crate::args::OutputArgs;

#[derive(Debug, Parser)]
pub struct Fetch {
    #[clap(flatten)]
    output: OutputArgs,
    #[clap(help = "The Starknet RPC provider")]
    rpc_url: Url,
    #[clap(help = "The Contract address")]
    contract_address: String,
}

impl Fetch {
    pub async fn run(self) -> Result<()> {
        let provider = AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(self.rpc_url)));
        let address = FieldElement::from_hex_be(&self.contract_address)?;

        let class = provider
            .get_class_at(BlockId::Tag(BlockTag::Latest), address)
            .await?;

        match class {
            ContractClass::Sierra(sierra) => {
                let abi: Vec<AbiEntry> = serde_json::from_str(&sierra.abi)?;
                self.output.write(&abi)
            },
            ContractClass::Legacy(_) => Err(anyhow!("Legacy class is not supported"))
        }
    }
}
