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
use starknet::accounts::{Account, SingleOwnerAccount};
use starknet::signers::{LocalWallet, SigningKey};

abigen!(ContractA, "./test.abi.json");

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = Url::parse("http://0.0.0.0:5050")?;
    let provider = AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));
    let provider2 = AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url)));

    let chain_id = provider.chain_id().await?;

    let account_address = FieldElement::from_hex_be(
        "0x3ee9e18edc71a6df30ac3aca2e0b02a198fbce19b7480a63a0d71cbd76652e0").unwrap();
    let signer = wallet_from_private_key(&Some("0x300001800000000300000180000000000030000000000003006001800006600".to_string())).unwrap();    
    // let account = SingleOwnerAccount::new(&provider, signer, account_address, chain_id);

    let contract_address = FieldElement::ZERO;
    let contract_caller = ContractA::new_caller(contract_address, provider).await?;
    let contract_invoker = ContractA::new_invoker(contract_address, provider2, account_address, signer).await?;

    contract_caller.hello_world(FieldElement::ZERO).await.expect("Fail call hello world");

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

// TODO: Check the class between cairo option and std option.
// we need to pre-declare option in cairo-types and use it with the path cairo_types::Option.
/// Returns a local wallet from a private key, if provided.
fn wallet_from_private_key(private_key: &std::option::Option<String>) -> std::option::Option<LocalWallet> {
    if let Some(pk) = private_key {
        let private_key = match FieldElement::from_hex_be(pk) {
            Ok(p) => p,
            Err(e) => {
                println!("Error importing private key: {:?}", e);
                return None;
            }
        };
        let key = SigningKey::from_secret_scalar(private_key);
        Some(LocalWallet::from_signing_key(key))
    } else {
        None
    }
}
