# Starknet abigen for rust bindings

This exploratory work aims at generating rust bindings from a contract ABI.

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

