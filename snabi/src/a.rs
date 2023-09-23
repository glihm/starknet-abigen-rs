// WARNING: This file is auto-generated.

use abigen_macro::abigen;
use cairo_types::CairoType;
use starknet::core::types::{BlockId, BlockTag};
use starknet::providers::Provider;
use starknet::accounts::Account;
abigen!(ContractA,
r#"
[
  {
    "type": "struct",
    "name": "core::starknet::eth_address::EthAddress",
    "members": [
      {
        "name": "address",
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
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "b",
        "type": "core::starknet::eth_address::EthAddress"
      },
      {
        "name": "c",
        "type": "core::starknet::class_hash::ClassHash"
      }
    ],
    "outputs": [],
    "state_mutability": "external"
  },
  {
    "type": "function",
    "name": "func2",
    "inputs": [],
    "outputs": [
      {
        "type": "(core::felt252, core::felt252)"
      }
    ],
    "state_mutability": "view"
  },
  {
    "type": "function",
    "name": "func3",
    "inputs": [],
    "outputs": [
      {
        "type": "core::array::Array::<core::felt252>"
      }
    ],
    "state_mutability": "view"
  },
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
    "name": "core::array::Span::<core::integer::u256>",
    "members": [
      {
        "name": "snapshot",
        "type": "@core::array::Array::<core::integer::u256>"
      }
    ]
  },
  {
    "type": "struct",
    "name": "ahah::c1::MyStruct2::<core::felt252>",
    "members": [
      {
        "name": "val",
        "type": "core::felt252"
      },
      {
        "name": "val2",
        "type": "core::array::Span::<core::integer::u256>"
      }
    ]
  },
  {
    "type": "function",
    "name": "func4",
    "inputs": [
      {
        "name": "a",
        "type": "ahah::c1::MyStruct2::<core::felt252>"
      }
    ],
    "outputs": [
      {
        "type": "core::array::Array::<core::integer::u8>"
      }
    ],
    "state_mutability": "view"
  },
  {
    "type": "event",
    "name": "ahah::c1::Event",
    "kind": "enum",
    "variants": []
  }
]
"#);
