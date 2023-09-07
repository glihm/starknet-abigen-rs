use abigen_macro::abigen;
use anyhow::Result;
use cairo_types::ty::CairoType;
use cairo_types::Result as CairoResult;
use serde_json;
use starknet::core::types::contract::AbiEntry;
use starknet::core::types::*;
use starknet::providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider};
use std::collections::HashMap;
use tokio::sync::RwLock as AsyncRwLock;
use url::Url;

abigen!(ContractA, "./test.abi.json");

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = Url::parse("http://0.0.0.0:5050")?;
    let provider = AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url)));

    let address = FieldElement::ZERO;
    let contract = ContractA::new(address, provider);

    let pg = PG {
        v1: FieldElement::THREE,
        v2: 1122_u128,
    };

    let test_enum = TestEnum::V1(FieldElement::THREE);

    assert_eq!(TestEnum::serialized_size(&test_enum), 2);
    assert_eq!(
        TestEnum::serialize(&test_enum),
        vec![FieldElement::ZERO, FieldElement::THREE]
    );
    let test_enum_des = vec![FieldElement::ZERO, FieldElement::THREE];
    assert_eq!(TestEnum::deserialize(&test_enum_des, 0).unwrap(), test_enum);

    assert_eq!(PG::serialized_size(&pg), 2);
    assert_eq!(
        PG::serialize(&pg),
        vec![FieldElement::THREE, 1122_u128.into()]
    );

    let felts = vec![FieldElement::THREE, 1122_u128.into()];
    assert_eq!(PG::deserialize(&felts, 0).unwrap(), pg);

    let v = vec![FieldElement::ONE, FieldElement::TWO];
    assert_eq!(Vec::<FieldElement>::serialized_size(&v), 3);

    let v = vec![FieldElement::ONE];
    assert_eq!(Vec::<FieldElement>::serialized_size(&v), 2);

    let v = vec![];
    assert_eq!(Vec::<FieldElement>::serialized_size(&v), 1);

    let vv = vec![vec![FieldElement::ONE, FieldElement::TWO]];
    assert_eq!(Vec::<Vec::<FieldElement>>::serialized_size(&vv), 4);

    let tt = TypesInTypes {
        a: vec![vec![FieldElement::TWO]],
        b: u256 { low: 1, high: 0 },
    };

    println!("{:?}", TypesInTypes::serialize(&tt).len());
    assert_eq!(TypesInTypes::serialized_size(&tt), 5);

    Ok(())
}

/*
    let mut calldata = vec![];
    calldata.extend(starknet::core::types::FieldElement::serialize(&FieldElement::ONE));
    calldata.extend(starknet::core::types::FieldElement::serialize(&FieldElement::TWO));

    let r = provider
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be("0x1234").unwrap(),
                entry_point_selector: starknet::macros::selector!("my_func"),
                calldata,
            },
            BlockId::Tag(BlockTag::Pending),
        )
        .await?;
*/
