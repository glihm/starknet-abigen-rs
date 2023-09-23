// WARNING: This file is auto-generated.

use abigen_macro::abigen;
use cairo_types::CairoType;
use starknet::accounts::Account;
use starknet::core::types::{BlockId, BlockTag};
use starknet::providers::Provider;
abigen!(
    GenContract,
    r#"
[
  {
    "type": "struct",
    "name": "contracts::gen::gen::MyStruct::<core::felt252>",
    "members": [
      {
        "name": "f1",
        "type": "core::felt252"
      },
      {
        "name": "f2",
        "type": "core::felt252"
      },
      {
        "name": "f3",
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
        "type": "contracts::gen::gen::MyStruct::<core::felt252>"
      }
    ],
    "outputs": [],
    "state_mutability": "external"
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
    "name": "contracts::gen::gen::MyStruct::<core::integer::u256>",
    "members": [
      {
        "name": "f1",
        "type": "core::felt252"
      },
      {
        "name": "f2",
        "type": "core::integer::u256"
      },
      {
        "name": "f3",
        "type": "core::felt252"
      }
    ]
  },
  {
    "type": "function",
    "name": "func2",
    "inputs": [
      {
        "name": "a",
        "type": "contracts::gen::gen::MyStruct::<core::integer::u256>"
      }
    ],
    "outputs": [],
    "state_mutability": "external"
  },
  {
    "type": "function",
    "name": "read",
    "inputs": [],
    "outputs": [
      {
        "type": "(core::felt252, core::felt252)"
      }
    ],
    "state_mutability": "view"
  },
  {
    "type": "event",
    "name": "contracts::gen::gen::Event",
    "kind": "enum",
    "variants": []
  }
]
"#
);
