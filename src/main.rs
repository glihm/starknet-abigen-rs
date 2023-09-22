mod tests;

use abigen_macro::abigen;
use anyhow::Result;
use cairo_types::ty::CairoType;
use cairo_types::types::starknet::*;


use url::Url;
use starknet::accounts::{Account, SingleOwnerAccount};
use starknet::core::types::*;
use starknet::macros::felt;
use starknet::providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};

abigen!(
    ContractA,
    r#"
[{"type":"struct","name":"core::starknet::eth_address::EthAddress","members":[{"name":"address","type":"core::felt252"}]},{"type":"function","name":"func1","inputs":[{"name":"a","type":"core::starknet::contract_address::ContractAddress"},{"name":"b","type":"core::starknet::eth_address::EthAddress"},{"name":"c","type":"core::starknet::class_hash::ClassHash"}],"outputs":[],"state_mutability":"external"},{"type":"function","name":"func2","inputs":[],"outputs":[{"type":"(core::felt252, core::felt252)"}],"state_mutability":"view"},{"type":"function","name":"func3","inputs":[],"outputs":[{"type":"core::array::Array::<core::felt252>"}],"state_mutability":"view"},{"type":"struct","name":"core::integer::u256","members":[{"name":"low","type":"core::integer::u128"},{"name":"high","type":"core::integer::u128"}]},{"type":"struct","name":"core::array::Span::<core::integer::u256>","members":[{"name":"snapshot","type":"@core::array::Array::<core::integer::u256>"}]},{"type":"struct","name":"ahah::c1::MyStruct2::<core::felt252>","members":[{"name":"val","type":"core::felt252"},{"name":"val2","type":"core::array::Span::<core::integer::u256>"}]},{"type":"function","name":"func4","inputs":[{"name":"a","type":"ahah::c1::MyStruct2::<core::felt252>"}],"outputs":[{"type":"core::array::Array::<core::integer::u8>"}],"state_mutability":"view"},{"type":"event","name":"ahah::c1::Event","kind":"enum","variants":[]}]
"#);

// abigen!(
//     ContractB,
//     r#"
// [
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
//     "name": "contracts::c1::OneGen::<core::felt252>",
//     "members": [
//       {
//         "name": "a",
//         "type": "core::felt252"
//       },
//       {
//         "name": "b",
//         "type": "core::felt252"
//       }
//     ]
//   },

//   {
//     "type": "struct",
//     "name": "contracts::c1::OneGen::<core::integer::u256>",
//     "members": [
//       {
//         "name": "a",
//         "type": "core::integer::u256"
//       },
//       {
//         "name": "b",
//         "type": "core::felt252"
//       }
//     ]
//   },

//   {
//     "type": "struct",
//     "name": "contracts::c1::TwoGen::<core::felt252, core::integer::u256>",
//     "members": [
//       {
//         "name": "a",
//         "type": "core::felt252"
//       },
//       {
//         "name": "b",
//         "type": "core::integer::u256"
//       }
//     ]
//   },

//   {
//     "type": "struct",
//     "name": "contracts::c1::EmbeddedGen::<core::felt252>",
//     "members": [
//       {
//         "name": "a",
//         "type": "core::felt252"
//       },
//       {
//         "name": "b",
//         "type": "contracts::c1::OneGen::<core::felt252>"
//       }
//     ]
//   },

//   {
//     "type": "enum",
//     "name": "contracts::c1::MyEnum",
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
//         "type": "()"
//       }
//     ]
//   },

//   {
//     "type": "enum",
//     "name": "contracts::c1::EnumGen::<core::felt252>",
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
//     "type": "struct",
//     "name": "contracts::c1::TestTupleGen::<(core::felt252, core::felt252)>",
//     "members": [
//       {
//         "name": "a",
//         "type": "(core::felt252, core::felt252)"
//       },
//       {
//         "name": "b",
//         "type": "core::felt252"
//       }
//     ]
//   },

//   {
//     "type": "function",
//     "name": "get_val",
//     "inputs": [],
//     "outputs": [
//       {
//         "type": "contracts::c1::EnumGen::<core::felt252>"
//       }
//     ],
//     "state_mutability": "view"
//   }
// ]
// "#
// );

#[tokio::main]
async fn main() -> Result<()> {

    // let me = MyEnum::V2(123_u128);
    // let felts = MyEnum::serialize(&me);
    // let me2 = MyEnum::deserialize(&felts, 0).unwrap();
    // assert_eq!(me, me2);
    // println!("{:?}", felts);

    // let meg = EnumGen::V3(vec![FieldElement::ZERO, FieldElement::ONE]);
    // let felts = EnumGen::<FieldElement>::serialize(&meg);    
    // let meg2 = EnumGen::<FieldElement>::deserialize(&felts, 0).unwrap();
    // assert_eq!(meg, meg2);
    // println!("{:?}", felts);

    // let og = OneGen {
    //     a: 1_u8,
    //     b: FieldElement::TWO,
    // };

    // let felts = OneGen::<u8>::serialize(&og);
    // println!("{:?}", felts);

    // let og = OneGen {
    //     a: u256 {
    //         low: 2,
    //         high: 0,
    //     },
    //     b: FieldElement::THREE,
    // };

    // let felts = OneGen::<u256>::serialize(&og);
    // println!("{:?}", felts);

    // let tg = TwoGen {
    //     a: 1_u8,
    //     b: u256 {
    //         low: 255,
    //         high: 0,
    //     }
    // };

    // let felts = TwoGen::<u8, u256>::serialize(&tg);
    // println!("{:?}", felts);

    // let tg = TwoGen {
    //     a: u256 {
    //         low: 1,
    //         high: 0,
    //     },
    //     b: FieldElement::MAX
    // };

    // let felts = TwoGen::<u256, FieldElement>::serialize(&tg);
    // let tg2 = TwoGen::<u256, FieldElement>::deserialize(&felts, 0).unwrap();
    // assert_eq!(tg, tg2);
    // println!("{:?}", felts);


    let rpc_url = Url::parse("http://0.0.0.0:5050")?;
    let provider =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));

    let account_address =
        felt!("0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973");

    let signer = wallet_from_private_key(&Some(
        "0x0000001800000000300000180000000000030000000000003006001800006600".to_string(),
    ))
    .unwrap();

    // If you modify the contract, even with a salt, it will be deployed at
    // a different address.
    let contract_address =
        felt!("0x032be4f29633d261254b1b1c6e7a6889a55354b665a513ef3928409303905631");

    let chain_id = provider.chain_id().await?;
    let account = SingleOwnerAccount::new(&provider, signer, account_address, chain_id);

    let contract_a = ContractA::new(contract_address, &provider).with_account(&account);

    // let m1 = MyStruct {
    //     val: FieldElement::ONE,
    //     val2: felt!("0xffff"),
    // };

    contract_a
        .func1(&ContractAddress(felt!("0x1234")), &EthAddress(felt!("0x222")), &ClassHash(felt!("0x444")))
        .await
        .expect("Fail invoke func1");

    let r = contract_a
        .func2()
        .await
        .expect("Fail call func2");

    println!("\n\n**\n\n{:?}", r);

    let r2 = contract_a
        .func3()
        .await
        .expect("Fail call func3");

    println!("\n\n**\n\n{:?}", r2);
    // let v_get_a = contract_a.get_val().await.expect("Fail call get val");
    // assert_eq!(v_get_a, FieldElement::TWO);
    // let v_get_b = contract_b.get_val().await.expect("Fail call get val");
    // assert_eq!(v_get_b, v_get_a);


    // let contract_b = ContractB::new(contract_address, &provider);

    // contract_a
    //     .set_val(FieldElement::TWO)
    //     .await
    //     .expect("Fail call set val");
    // let v_get_a = contract_a.get_val().await.expect("Fail call get val");
    // assert_eq!(v_get_a, FieldElement::TWO);
    // let v_get_b = contract_b.get_val().await.expect("Fail call get val");
    // assert_eq!(v_get_b, v_get_a);

    // contract_a
    //     .hello_world(FieldElement::THREE)
    //     .await
    //     .expect("Fail call hello world");

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
