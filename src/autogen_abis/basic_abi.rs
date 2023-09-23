// WARNING: This file is auto-generated.

use abigen_macro::abigen;
use cairo_types::CairoType;
use starknet::accounts::Account;
use starknet::core::types::{BlockId, BlockTag};
use starknet::providers::Provider;
abigen!(
    BasicContract,
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
    "type": "function",
    "name": "set_storage",
    "inputs": [
      {
        "name": "v1",
        "type": "core::felt252"
      },
      {
        "name": "v2",
        "type": "core::integer::u256"
      }
    ],
    "outputs": [],
    "state_mutability": "external"
  },
  {
    "type": "function",
    "name": "read_storage_tuple",
    "inputs": [],
    "outputs": [
      {
        "type": "(core::felt252, core::integer::u256)"
      }
    ],
    "state_mutability": "view"
  },
  {
    "type": "event",
    "name": "contracts::basic::basic::Event",
    "kind": "enum",
    "variants": []
  }
]
"#
);
