# Starknet abigen for rust bindings

This exploratory work aims at generating rust bindings from a contract ABI.

## Quick start

1. Terminal 1: Run Katana

```sh
dojoup -v nightly
```

```sh
katana
```

2.  Terminal 2: Contracts setup

```sh
cd crates/contracts && scarb build && make setup
```

```sh
cargo run
```

## Cairo - Rust similarity

We've tried to leverage the similarity between Rust and Cairo.
With this in mind, the bindings are generated to be as natural as possible from a Rust perspective.

So most of the types are Rust types, and the basic value for us is the `FieldElement` from `starknet-rs`.
Except few exceptions like `ContractAddress, ClassHash and EthAddress`, which a custom structs to map those
Cairo native type, all the types are mapped to native Rust types.

```rust
// Cairo: fn get_data(self: @ContractState) -> Span<felt252>
fn get_data() -> Vec<FieldElement>

// Cairo: fn get_opt(self: @ContractState, val: u32) -> Option<felt252>
fn get_opt(val: u32) -> Option<FieldElement>

// Cairo: struct MyData { a: felt252, b: u32, c: Span<u32> }
struct MyData {
  a: FieldElement,
  b: u32,
  c: Vec<u32>,
}
```

If you want to leverage the (de)serialization generated by the bindings, to make raw calls with `starknet-rs`, you can:

```rust
let d = MyData {
  a: FieldElement::TWO,
  b: 123_u32,
  c: vec![8, 9],
};

let felts = MyData::serialize(&d);

let felts = vec![FieldElement::ONE, FieldElement::TWO];
// For now you have to provide the index. Later an other method will consider deserialization from index 0.
let values = Vec::<u32>::deserialize(felts, 0).unwrap;
```

Any type implementing the `CairoType` trait can be used this way.

Supported types as built-in in cairo-types:

* `u8,16,32,64,128`
* `i8,16,32,64,128`
* `tuple size 2`
* `Span/Array` -> `Vec`
* `ClassHash`
* `ContractAddress`
* `EthAddress`
* `Option`
* `Result`
* `unit`

Any struct/enum in the ABI that use those types or inner struct/enum
that uses those types will work.

## Generate the binding for your contracts

1. If you have a large ABI, consider adding a file (at the same level of your `Cargo.toml`) with the `JSON` containing the ABI.
   Then you can load the whole file using:

```rust
abigen!(MyContract, "./mycontract.abi.json")
```

2. If you only want to make a quick call without too much setup, you can paste an ABI directly using:

```rust
abigen!(MyContract, r#"
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
  }
]
"#);
```

## Initialize the contract

In starknet, we also have `call` and `invoke`. A `call` doesn't alter the state, and hence does not require an account + private key to sign.
An `invoke` requires you to provide an account and a private key to sign and send the transaction.

```rust
use abigen_macro::abigen;
use anyhow::Result;
use cairo_types::ty::CairoType;

use starknet::accounts::{Account, SingleOwnerAccount};

use starknet::core::types::*;
use starknet::providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};

abigen!(MyContract, "./mycontract.abi.json")

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = Url::parse("http://0.0.0.0:5050")?;

    let provider =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));

    let contract_address = felt!("0x0546a164c8d10fd38652b6426ef7be159965deb9a0cbf3e8a899f8a42fd86761");

     // Call.
    let my_contract = MyContract::new(contract_address, &provider);
    let val = my_contract.get_val().await?;

     let account_address = felt!("0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973");

    let signer = wallet_from_private_key(&Some(
        "0x0000001800000000300000180000000000030000000000003006001800006600".to_string(),
    )).unwrap();
    let account = SingleOwnerAccount::new(&provider, signer, account_address, chain_id);

    // Invoke.
    let mycontract = MyContract::new(contract_address, &provider).with_account(&account).await?;

    mycontract.set_val(FieldElement::TWO).await?;
}

// Util function to create a LocalWallet.
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
```

This way of initializing the contract is not the final one, feel free to propose alternative in the issues.

## Considerations

On Starknet, a contract's ABI is a flat representation of all the types and functions associated with the contract.

Each `struct` or `enum` that are used by external functions of the contracts are embedded in the ABI, which ensure a full description of the types, self-contained in a single ABI file.

Cairo has the capability of using generic types. However, the ABI does not reflect this
implementation detail.

```rust
struct MyStruct<T> {
    a: T,
    b: u8,
}

// This struct in the ABI will be flatten depending on the impl found in the code.

(...)

fn func_1(ms: MyStruct<felt252>)
```

```json
// This function has the `felt252` impl, so the ABI will contain:

  {
    "type": "struct",
    "name": "package::contract1::MyStruct",
    "members": [
      {
        "name": "a",
        "type": "core::felt252"
      },
      {
        "name": "b",
        "type": "core::integer::u8"
      }
    ]
  },
```

We don't have the possibility to know which type was impl by the generic type `T` only looking at the ABI.

## Serialization

Cairo serializes everything as `felt252`. Some edge cases to have in mind:

1. Enum

Enumerations are serialized with the index of the variant first, and then the value (is any).

```rust
enum MyEnum {
    V1: u128,
    V2,
}

let a = MyEnum::V1(2_u128);
let b = MyEnum::V2;
```

Will be serialized as:

```
a: [0, 2]
b: [1]
```

2. Span/Array

After serialization, `Span` and `Array` are processed in the same fashion.
The length is serialized first, and then the following elements.

```rust
let a = array![];
let b = array![1, 2];
```

Will be serialized as:

```
a: [0]
b: [2, 1, 2]
```

3. Struct

`struct` are serialized as their fields define it. There is no length at the beginning. It depends on the fields order.

```rust
struct MyStruct {
    a: felt252,
    b: u256,
    c: Array<felt252>,
}

let s = MyStruct {
    a: 123,
    b: 1_u256,
    c: array![9],
}
```

Will be serialized as:

```
[123, 1, 0, 1, 9]
```

## Current design idea

At first, we tried to take inspiration from `alloy`, the new implementation for Ethereum rust library.

But cairo differs from solidity in several aspect, starting with generic types. So ideally, we should be able to tokenize the ABI into `syn` to then totally control how we want to lower the detected types.

But for now, the approach is inspired from `alloy`, but simpler and more hand made for type parsing.

1. First, we have the `CairoType` (which may be renamed as `CairoSerializeable`) trait. This trait defines how a rust type is serialized / deserialized as Cairo `FieldElement` from `starknet-rs`.

2. Then, `AbiType` is able to parse any cairo type, even if it's nested. As we have to be able to express how types are nested to ensure the correct serialization.

3. After having the `AbiType`, we then want to expand in a macro the types and their serialization logic in a macro. For that, each of the `AbiEntry` that are `struct`, `enum`, `function` must be expanded using the `AbiType` info to correctly generate the serialization code.

4. Finally, the contract itself, must be generated with the `provider` already internalized, to easily do some `invoke` and `calls`, using pure rust types.

## Disclaimer

This is a very early stage of the project. The idea is to have a first version that can be revised by the community and then enhanced.

Hopefully one day we can have a great lib that can be integrated to `starknet-rs` or remain a stand alone crate which can be combined with `starknet-rs`.
