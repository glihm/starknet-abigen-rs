use abigen_macro::abigen;
use anyhow::Result;
use cairo_types::ty::CairoType;

use starknet::accounts::Account;

use starknet::core::types::*;
use starknet::providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};

use url::Url;

// Both working fine
abigen!(ContractA, "./test.abi.json");
// abigen!(
//     ContractA,
//     r#"[
//   {
//     "type": "function",
//     "name": "get_val",
//     "inputs": [],
//     "outputs": [
//       {
//         "type": "core::felt252"
//       }
//     ],
//     "state_mutability": "view"
//   },
//   {
//     "type": "function",
//     "name": "set_val",
//     "inputs": [
//       {
//         "name": "v",
//         "type": "core::felt252"
//       }
//     ],
//     "outputs": [],
//     "state_mutability": "external"
//   },
//   {
//     "type": "struct",
//     "name": "contracts::c1::PG",
//     "members": [
//       {
//         "name": "v1",
//         "type": "core::felt252"
//       },
//       {
//         "name": "v2",
//         "type": "core::integer::u128"
//       }
//     ]
//   },
//   {
//     "type": "function",
//     "name": "hello_world",
//     "inputs": [
//       {
//         "name": "value",
//         "type": "core::felt252"
//       }
//     ],
//     "outputs": [
//       {
//         "type": "contracts::c1::PG"
//       }
//     ],
//     "state_mutability": "view"
//   },
//   {
//     "type": "struct",
//     "name": "core::integer::u256",
//     "members": [
//       {
//         "name": "low",
//         "type": "core::integer::u128"
//       },
//       {
//         "name": "high",
//         "type": "core::integer::u128"
//       }
//     ]
//   },
//   {
//     "type": "struct",
//     "name": "contracts::c1::InnerOne",
//     "members": [
//       {
//         "name": "a",
//         "type": "core::integer::u256"
//       },
//       {
//         "name": "b",
//         "type": "core::array::Array::<core::felt252>"
//       }
//     ]
//   },
//   {
//     "type": "struct",
//     "name": "contracts::c1::ComplexOne",
//     "members": [
//       {
//         "name": "pg",
//         "type": "contracts::c1::PG"
//       },
//       {
//         "name": "inner",
//         "type": "contracts::c1::InnerOne"
//       }
//     ]
//   },
//   {
//     "type": "function",
//     "name": "call_test",
//     "inputs": [
//       {
//         "name": "pg",
//         "type": "contracts::c1::PG"
//       },
//       {
//         "name": "inner",
//         "type": "contracts::c1::InnerOne"
//       }
//     ],
//     "outputs": [
//       {
//         "type": "contracts::c1::ComplexOne"
//       }
//     ],
//     "state_mutability": "external"
//   },
//   {
//     "type": "struct",
//     "name": "contracts::c1::TypesInTypes::<core::integer::u256>",
//     "members": [
//       {
//         "name": "a",
//         "type": "core::array::Array::<core::array::Array::<core::felt252>>"
//       },
//       {
//         "name": "b",
//         "type": "core::integer::u256"
//       }
//     ]
//   },
//   {
//     "type": "function",
//     "name": "call_bla",
//     "inputs": [
//       {
//         "name": "a",
//         "type": "contracts::c1::TypesInTypes::<core::integer::u256>"
//       }
//     ],
//     "outputs": [
//       {
//         "type": "contracts::c1::TypesInTypes::<core::integer::u256>"
//       }
//     ],
//     "state_mutability": "external"
//   },
//   {
//     "type": "struct",
//     "name": "core::array::Span::<core::felt252>",
//     "members": [
//       {
//         "name": "snapshot",
//         "type": "@core::array::Array::<core::felt252>"
//       }
//     ]
//   },
//   {
//     "type": "function",
//     "name": "call_bou",
//     "inputs": [],
//     "outputs": [
//       {
//         "type": "(core::array::Span::<core::felt252>, core::felt252)"
//       }
//     ],
//     "state_mutability": "external"
//   },
//   {
//     "type": "struct",
//     "name": "core::array::Span::<(core::felt252, contracts::c1::PG)>",
//     "members": [
//       {
//         "name": "snapshot",
//         "type": "@core::array::Array::<(core::felt252, contracts::c1::PG)>"
//       }
//     ]
//   },
//   {
//     "type": "function",
//     "name": "call_bii",
//     "inputs": [],
//     "outputs": [
//       {
//         "type": "(core::array::Span::<(core::felt252, contracts::c1::PG)>, core::felt252)"
//       }
//     ],
//     "state_mutability": "external"
//   },
//   {
//     "type": "enum",
//     "name": "contracts::c1::TestEnum::<core::felt252>",
//     "variants": [
//       {
//         "name": "V1",
//         "type": "core::felt252"
//       },
//       {
//         "name": "V2",
//         "type": "core::integer::u128"
//       },
//       {
//         "name": "V3",
//         "type": "core::array::Array::<core::felt252>"
//       },
//       {
//         "name": "V4",
//         "type": "()"
//       }
//     ]
//   },
//   {
//     "type": "function",
//     "name": "call_baba",
//     "inputs": [],
//     "outputs": [
//       {
//         "type": "contracts::c1::TestEnum::<core::felt252>"
//       }
//     ],
//     "state_mutability": "external"
//   },
//   {
//     "type": "function",
//     "name": "call_baba2",
//     "inputs": [],
//     "outputs": [
//       {
//         "type": "contracts::c1::TestEnum::<core::integer::u256>"
//       }
//     ],
//     "state_mutability": "external"
//   },
//   {
//     "type": "function",
//     "name": "call_pia",
//     "inputs": [],
//     "outputs": [
//       {
//         "type": "contracts::c1::TestEnum::<core::felt252>"
//       }
//     ],
//     "state_mutability": "external"
//   },
//   {
//     "type": "enum",
//     "name": "core::option::Option::<core::felt252>",
//     "variants": [
//       {
//         "name": "Some",
//         "type": "core::felt252"
//       },
//       {
//         "name": "None",
//         "type": "()"
//       }
//     ]
//   },
//   {
//     "type": "function",
//     "name": "call_bibi",
//     "inputs": [],
//     "outputs": [
//       {
//         "type": "core::option::Option::<core::felt252>"
//       }
//     ],
//     "state_mutability": "external"
//   },
//   {
//     "type": "function",
//     "name": "call_bobo",
//     "inputs": [],
//     "outputs": [
//       {
//         "type": "core::option::Option::<core::felt252>"
//       }
//     ],
//     "state_mutability": "external"
//   },
//   {
//     "type": "function",
//     "name": "call_bok",
//     "inputs": [],
//     "outputs": [
//       {
//         "type": "(core::felt252, core::felt252)"
//       }
//     ],
//     "state_mutability": "external"
//   },
//   {
//     "type": "event",
//     "name": "contracts::c1::Event",
//     "kind": "enum",
//     "variants": []
//   }
// ]
// "#
// );
abigen!(
    ContractB,
    r#"
        [
        {
            "type": "function",
            "name": "get_val",
            "inputs": [],
            "outputs": [
            {
                "type": "core::felt252"
            }
            ],
            "state_mutability": "view"
        }]
"#
);

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = Url::parse("http://0.0.0.0:5050")?;
    let provider =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));
    let provider2 =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));
    let provider3 =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));

    let _chain_id = provider.chain_id().await?;

    let account_address = FieldElement::from_hex_be(
        "0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973",
    )
    .unwrap();
    let signer = wallet_from_private_key(&Some(
        "0x0000001800000000300000180000000000030000000000003006001800006600".to_string(),
    ))
    .unwrap();

    let contract_address = FieldElement::from_hex_be(
        "0x0546a164c8d10fd38652b6426ef7be159965deb9a0cbf3e8a899f8a42fd86761",
    )
    .unwrap();

    let contract_caller_a = ContractA::new_caller(contract_address, provider).await?;
    let contract_caller_b = ContractB::new_caller(contract_address, provider3).await?;
    let contract_invoker =
        ContractA::new_invoker(contract_address, provider2, account_address, signer).await?;

    let pr = contract_caller_a
        .hello_world(FieldElement::THREE)
        .await
        .expect("Fail call hello world");
    assert_eq!(pr.v1, FieldElement::THREE);
    assert_eq!(pr.v2, 11_u128);

    contract_invoker
        .set_val(FieldElement::TWO)
        .await
        .expect("Fail call set val");
    let v_set = contract_caller_a
        .get_val()
        .await
        .expect("Fail call get val");
    assert_eq!(v_set, FieldElement::TWO);

    contract_invoker
        .hello_world(FieldElement::THREE)
        .await
        .expect("Fail call hello world");
    assert_eq!(pr.v1, FieldElement::THREE);
    assert_eq!(pr.v2, 11_u128);

    let pg = PG {
        v1: FieldElement::THREE,
        v2: 1122_u128,
    };

    // let test_u256 = u256 {
    //     low: 1122_u128,
    //     high: 1122_u128,
    // };

    // assert_eq!(u256::serialized_size(&test_u256), 2);
    // assert_eq!(
    //     u256::serialize(&test_u256),
    //     vec![1122_u128.into(), 1122_u128.into()]
    // );
    // let test_u256_des = vec![1122_u128.into(), 1122_u128.into()];
    // assert_eq!(u256::deserialize(&test_u256_des, 0).unwrap(), test_u256);

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
fn wallet_from_private_key(
    private_key: &std::option::Option<String>,
) -> std::option::Option<LocalWallet> {
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
