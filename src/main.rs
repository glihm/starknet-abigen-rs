use abigen_macro::abigen;
use anyhow::Result;
use serde_json;
use starknet::core::types::contract::AbiEntry;
use starknet::core::types::*;
use starknet::providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider};
use std::collections::HashMap;
use tokio::sync::RwLock as AsyncRwLock;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = Url::parse("http://0.0.0.0:5050")?;
    let provider = AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url)));

    abigen!(ContractA, "./test.abi.json");

    Ok(())
}
