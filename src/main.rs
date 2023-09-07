use abigen_macro::abigen;
use anyhow::Result;
use serde_json;
use starknet::core::types::contract::AbiEntry;
use starknet::core::types::*;
use starknet::providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider};
use std::collections::HashMap;
use tokio::sync::RwLock as AsyncRwLock;
use url::Url;
use cairo_types::ty::CairoType;
use cairo_types::Result as CairoResult;

abigen!(ContractA, "./test.abi.json");

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = Url::parse("http://0.0.0.0:5050")?;
    let provider = AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url)));

    let pg = PG {
        v1: FieldElement::THREE,
        v2: 1122_u128,
    };

    assert_eq!(PG::serialized_size(&pg), 2);
    assert_eq!(PG::serialize(&pg), vec![FieldElement::THREE, 1122_u128.into()]);

    let felts = vec![FieldElement::THREE, 1122_u128.into()];
    assert_eq!(PG::deserialize(&felts, 0).unwrap(), pg);

    Ok(())
}
