# Starknet abigen for rust bindings

Passionate work about providing rust binding to Starknet community, by the community.

The current state of the repo is still very experimental, but we are
reaching a first good milestrone.

- [X] Types generation with serialization/deserialization for any type in the contract.
- [X] Support for generic types.
- [X] Auto generation of the contract with it's functions (call and invoke).
- [ ] Generation of Events structs to parse automatically `EmittedEvent`.

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
cd contracts && scarb build && make setup
```

```sh
cargo run
```

## Overview

This repository contains the following crates:

- [`abigen-macro`] - The [`abigen!`] procedural macro
- [`cairo-types`] - Cairo types binded with Rust types
- [`cairo-type-parser`] - A simple parser for Cairo type strings

[`abigen-macro`]: ./crates/abigen-macro
[`cairo-types`]: ./crates/cairo-types
[`cairo-type-parser`]: ./crates/cairo-type-parser
[`abigen!`]: ./crates/abigen-macro

## Cairo - Rust similarity

We've tried to leverage the similarity between Rust and Cairo.
With this in mind, the bindings are generated to be as natural as possible from a Rust perspective.

So most of the types are Rust types, and the basic value for us is the `FieldElement` from `starknet-rs`.

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

3. You can use `snabi` tool that is inside the repo to generate a rust file containing your full ABI expanded with the macro. You can find an example of this in the `src/main.rs`.

Basically you can:
```bash
# Generate a rust file directly from a Sierra file to then simply import it as a module in Rust.
cargo run -p snabi from-sierra \
    target/dev/contracts_basic.sierra.json \
    --expandable /path/my_contract_abi.rs \
    --name MyContract

# You can also fetch an ABI on chain:
cargo run -p snabi fetch \
    http://0.0.0.0:5050 \
    0x032be4f29633d261254b1b1c6e7a6889a55354b665a513ef3928409303905631 \
    --expandable /path/my_contract_abi.rs \
    --name MyContract
```

## How to work with events

Events are special structs/enum that we usually want to deserialize effectively.
The `abigen!` macro generate all the events associated types, and this always include
one enum always named `Event`.

Any contract you use `abigen!` on will contain this enum, and this also includes the convertion
from `EmittedEvent`, which is the `starknet-rs` type returned when we fetch events.

So you can do this:

```rust
// the Event enum is always declared if at least 1 event is present
// in the cairo file.
use myContract::{Event as AnyEvent};

let events = provider.fetch_events(...);

for e in events {
    // The `TryFrom` is already implemented for each variant, which includes
    // the deserialization of the variant.
    let my_event: AnyEvent = match e.try_into() {
        Ok(ev) => ev,
        Err(_s) => {
            // An event from other contracts, ignore.
            continue;
        }
    };
    
    // Then, you can safely check which event it is, and work with it,
    // with the rust type!
    match my_event {
        AnyEvent::MyEventA(a) => {
            // do stuff with a.
        }
        AnyEvent::MyEventB(b) => {
            // do stuff with b.
        }
        ...
    };
}
```

## Example

This is an example of how easy is to call a contract. You can find this code [here](https://github.com/glihm/starknet-abigen-rs/blob/main/src/main.rs):
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let (provider, account) = katana_default::get_provider_and_account().await?;

    let basic = BasicContract::new(
        felt!("0x04383de1eb63b223170e1de699ff5074fbc1f6096e14604615b65d3d1cc28c7d"),
        Arc::clone(&provider),
    )
    .with_account(Arc::clone(&account));

    let v1 = FieldElement::ONE;
    let v2 = u256 {
        low: 2_u128,
        high: 0_u128,
    };

    basic.set_storage(&v1, &v2).await?;

    let (v1_r, v2_r) = basic.read_storage_tuple().await.unwrap();
    assert_eq!(v1_r, v1);
    assert_eq!(v2_r, v2);

    let gen = GenContract::new(
        felt!("0x0505ca46219e39ede6f186e3056535d82e4eb44bbb49b77531930eeacd1c89e3"),
        Arc::clone(&provider),
    )
    .with_account(Arc::clone(&account));

    let ms = MyStruct {
        f1: FieldElement::ONE,
        f2: FieldElement::TWO,
        f3: FieldElement::THREE,
    };

    gen.func1(&ms).await?;

    let (f1, f2) = gen.read().await.unwrap();
    assert_eq!(f1, FieldElement::ONE);
    assert_eq!(f2, FieldElement::TWO);

    Ok(())
}
```

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

Will be serialized like this, with enum variant index first:

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

## Disclaimer

This is a very early stage of the project. The idea is to have a first version that can be revised by the community and then enhanced.

Hopefully one day we can have a great lib that can be integrated to `starknet-rs` or remain a stand alone crate which can be combined with `starknet-rs`.

## Credits

None of these crates would have been possible without the great work done in:

- [`ethers-rs`](https://github.com/gakonst/ethers-rs/)
- [`alloy-rs`](https://github.com/alloy-rs/core/)
- [`starknet-rs`](https://github.com/xJonathanLEI/starknet-rs/)
