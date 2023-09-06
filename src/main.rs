use anyhow::Result;
use tokio::sync::RwLock as AsyncRwLock;
use starknet::core::types::*;
use starknet::core::types::contract::AbiEntry;
use serde_json;
use starknet::providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider};
use abigen_macro::abigen;
use url::Url;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = Url::parse("http://0.0.0.0:5050")?;
    let provider = AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url)));

    abigen!(ContractA, "./test.abi.json");

    Ok(())
}

