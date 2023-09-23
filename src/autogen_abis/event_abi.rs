// WARNING: This file is auto-generated.

use abigen_macro::abigen;
use cairo_types::CairoType;
use starknet::accounts::Account;
abigen!(EventContract,
r#"
[
  {
    "type": "function",
    "name": "read",
    "inputs": [],
    "outputs": [
      {
        "type": "core::felt252"
      }
    ],
    "state_mutability": "external"
  },
  {
    "type": "struct",
    "name": "core::array::Span::<core::felt252>",
    "members": [
      {
        "name": "snapshot",
        "type": "@core::array::Array::<core::felt252>"
      }
    ]
  },
  {
    "type": "function",
    "name": "emit_a",
    "inputs": [
      {
        "name": "header",
        "type": "core::felt252"
      },
      {
        "name": "value",
        "type": "core::array::Span::<core::felt252>"
      }
    ],
    "outputs": [],
    "state_mutability": "external"
  },
  {
    "type": "function",
    "name": "emit_b",
    "inputs": [
      {
        "name": "value",
        "type": "core::felt252"
      }
    ],
    "outputs": [],
    "state_mutability": "external"
  },
  {
    "type": "function",
    "name": "emit_c",
    "inputs": [
      {
        "name": "v1",
        "type": "core::felt252"
      },
      {
        "name": "v2",
        "type": "core::felt252"
      },
      {
        "name": "v3",
        "type": "core::felt252"
      },
      {
        "name": "v4",
        "type": "core::starknet::contract_address::ContractAddress"
      }
    ],
    "outputs": [],
    "state_mutability": "external"
  },
  {
    "type": "event",
    "name": "contracts::event::event::MyEventA",
    "kind": "struct",
    "members": [
      {
        "name": "header",
        "type": "core::felt252",
        "kind": "key"
      },
      {
        "name": "value",
        "type": "core::array::Span::<core::felt252>",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "contracts::event::event::MyEventB",
    "kind": "struct",
    "members": [
      {
        "name": "value",
        "type": "core::felt252",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "contracts::event::event::MyEventC",
    "kind": "struct",
    "members": [
      {
        "name": "v1",
        "type": "core::felt252",
        "kind": "key"
      },
      {
        "name": "v2",
        "type": "core::felt252",
        "kind": "key"
      },
      {
        "name": "v3",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "v4",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "contracts::event::event::Event",
    "kind": "enum",
    "variants": [
      {
        "name": "MyEventA",
        "type": "contracts::event::event::MyEventA",
        "kind": "nested"
      },
      {
        "name": "MyEventB",
        "type": "contracts::event::event::MyEventB",
        "kind": "nested"
      },
      {
        "name": "MyEventC",
        "type": "contracts::event::event::MyEventC",
        "kind": "nested"
      }
    ]
  }
]
"#);
