mod tests;

use abigen_macro::abigen;
use anyhow::Result;
use cairo_types::ty::CairoType;

use starknet::accounts::{Account};

use starknet::core::types::*;


use starknet::signers::{LocalWallet, SigningKey};

use starknet::providers::Provider;

abigen!(
    ContractA,
    r#"
[
  {
    "type": "struct",
    "name": "ahah::c1::MyStruct::<core::felt252>",
    "members": [
      {
        "name": "val",
        "type": "core::felt252"
      },
      {
        "name": "val2",
        "type": "core::felt252"
      }
    ]
  },
  {
    "type": "function",
    "name": "func1",
    "inputs": [
      {
        "name": "a",
        "type": "(core::felt252, ahah::c1::MyStruct::<core::felt252>)"
      }
    ],
    "outputs": [],
    "state_mutability": "external"
  },
  {
    "type": "event",
    "name": "ahah::c1::Event",
    "kind": "enum",
    "variants": []
  }
]
"#);

abigen!(
    ContractB,
    r#"
[
  {
    "type": "struct",
    "name": "core::integer::u256",
    "members": [
      {
        "name": "low",
        "type": "core::integer::u128"
      },
      {
        "name": "high",
        "type": "core::integer::u128"
      }
    ]
  },

  {
    "type": "struct",
    "name": "contracts::c1::OneGen::<core::felt252>",
    "members": [
      {
        "name": "a",
        "type": "core::felt252"
      },
      {
        "name": "b",
        "type": "core::felt252"
      }
    ]
  },

  {
    "type": "struct",
    "name": "contracts::c1::OneGen::<core::integer::u256>",
    "members": [
      {
        "name": "a",
        "type": "core::integer::u256"
      },
      {
        "name": "b",
        "type": "core::felt252"
      }
    ]
  },

  {
    "type": "struct",
    "name": "contracts::c1::TwoGen::<core::felt252, core::integer::u256>",
    "members": [
      {
        "name": "a",
        "type": "core::felt252"
      },
      {
        "name": "b",
        "type": "core::integer::u256"
      }
    ]
  },

  {
    "type": "struct",
    "name": "contracts::c1::EmbeddedGen::<core::felt252>",
    "members": [
      {
        "name": "a",
        "type": "core::felt252"
      },
      {
        "name": "b",
        "type": "contracts::c1::OneGen::<core::felt252>"
      }
    ]
  },

  {
    "type": "enum",
    "name": "contracts::c1::MyEnum",
    "variants": [
      {
        "name": "V1",
        "type": "core::felt252"
      },
      {
        "name": "V2",
        "type": "core::integer::u128"
      },
      {
        "name": "V3",
        "type": "()"
      }
    ]
  },

  {
    "type": "enum",
    "name": "contracts::c1::EnumGen::<core::felt252>",
    "variants": [
      {
        "name": "V1",
        "type": "core::felt252"
      },
      {
        "name": "V2",
        "type": "core::integer::u128"
      },
      {
        "name": "V3",
        "type": "core::array::Array::<core::felt252>"
      },
      {
        "name": "V4",
        "type": "()"
      }
    ]
  },

  {
    "type": "struct",
    "name": "contracts::c1::TestTupleGen::<(core::felt252, core::felt252)>",
    "members": [
      {
        "name": "a",
        "type": "(core::felt252, core::felt252)"
      },
      {
        "name": "b",
        "type": "core::felt252"
      }
    ]
  },

  {
    "type": "function",
    "name": "get_val",
    "inputs": [],
    "outputs": [
      {
        "type": "contracts::c1::EnumGen::<core::felt252>"
      }
    ],
    "state_mutability": "view"
  }
]
"#
);

#[tokio::main]
async fn main() -> Result<()> {

    let me = MyEnum::V2(123_u128);
    let felts = MyEnum::serialize(&me);
    let me2 = MyEnum::deserialize(&felts, 0).unwrap();
    assert_eq!(me, me2);
    println!("{:?}", felts);

    let meg = EnumGen::V3(vec![FieldElement::ZERO, FieldElement::ONE]);
    let felts = EnumGen::<FieldElement>::serialize(&meg);    
    let meg2 = EnumGen::<FieldElement>::deserialize(&felts, 0).unwrap();
    assert_eq!(meg, meg2);
    println!("{:?}", felts);

    let og = OneGen {
        a: 1_u8,
        b: FieldElement::TWO,
    };

    let felts = OneGen::<u8>::serialize(&og);
    println!("{:?}", felts);

    let og = OneGen {
        a: u256 {
            low: 2,
            high: 0,
        },
        b: FieldElement::THREE,
    };

    let felts = OneGen::<u256>::serialize(&og);
    println!("{:?}", felts);

    let tg = TwoGen {
        a: 1_u8,
        b: u256 {
            low: 255,
            high: 0,
        }
    };

    let felts = TwoGen::<u8, u256>::serialize(&tg);
    println!("{:?}", felts);

    let tg = TwoGen {
        a: u256 {
            low: 1,
            high: 0,
        },
        b: FieldElement::MAX
    };

    let felts = TwoGen::<u256, FieldElement>::serialize(&tg);
    let tg2 = TwoGen::<u256, FieldElement>::deserialize(&felts, 0).unwrap();
    assert_eq!(tg, tg2);
    println!("{:?}", felts);


    // let rpc_url = Url::parse("http://0.0.0.0:5050")?;
    // let provider =
    //     AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));

    // let account_address =
    //     felt!("0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973");

    // let signer = wallet_from_private_key(&Some(
    //     "0x0000001800000000300000180000000000030000000000003006001800006600".to_string(),
    // ))
    // .unwrap();

    // // If you modify the contract, even with a salt, it will be deployed at
    // // a different address.
    // let contract_address =
    //     felt!("0x02de662b356d56d25be451106ae2b54db05e476f8bbb9f0519fd8c2a63e575a9");

    // let chain_id = provider.chain_id().await?;
    // let account = SingleOwnerAccount::new(&provider, signer, account_address, chain_id);

    // let contract_a = ContractA::new(contract_address, &provider).with_account(&account);
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
