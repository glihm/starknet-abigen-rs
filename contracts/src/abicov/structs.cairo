//! A contract with structs.
use starknet::{ClassHash, ContractAddress, EthAddress};

#[derive(Serde, Drop)]
struct Simple {
    felt: felt252,
    uint256: u256,
    uint64: u64,
    address: ContractAddress,
    class_hash: ClassHash,
    eth_address: EthAddress,
    tuple: (felt252, u256),
    span: Span<felt252>,
}

#[derive(Serde, Drop)]
struct StructWithStruct {
    simple: Simple,
}

#[derive(Serde, Drop)]
struct GenericOne<T> {
    a: T,
    b: felt252,
    c: u256,
}

#[derive(Serde, Drop)]
struct GenericTwo<T, U> {
    a: T,
    b: U,
    c: felt252,
}

#[derive(Serde, Drop)]
struct GenericOfGeneric<T> {
    a: GenericOne<T>,
}

#[starknet::contract]
mod structs {
    use super::{Simple, StructWithStruct, GenericOne, GenericTwo, GenericOfGeneric};

    #[storage]
    struct Storage {}

    #[external(v0)]
    fn get_simple(self: @ContractState) -> Simple {
        Simple {
            felt: 1,
            uint256: 2_u256,
            uint64: 3_u64,
            address: 0x1234.try_into().unwrap(),
            class_hash: 0x1122.try_into().unwrap(),
            eth_address: 0x3344.try_into().unwrap(),
            tuple: (1, 2_u256),
            span: array![1, 2, 3, 4].span(),
        }
    }

    #[external(v0)]
    fn set_simple(ref self: ContractState, simple: Simple) {}

    #[external(v0)]
    fn get_struct_w_struct(self: @ContractState) -> StructWithStruct {
        StructWithStruct {
            simple: Simple {
                felt: 1,
                uint256: 2_u256,
                uint64: 3_u64,
                address: 0x1234.try_into().unwrap(),
                class_hash: 0x1122.try_into().unwrap(),
                eth_address: 0x3344.try_into().unwrap(),
                tuple: (1, 2_u256),
                span: array![1, 2, 3, 4].span(),
            }
        }
    }

    #[external(v0)]
    fn set_struct_w_struct(ref self: ContractState, sws: StructWithStruct) {}

    #[external(v0)]
    fn get_generic_one(self: @ContractState) -> GenericOne<felt252> {
        GenericOne { a: 1, b: 2, c: 3_u256, }
    }

    #[external(v0)]
    fn set_generic_one(ref self: ContractState, generic: GenericOne<u256>) {}

    #[external(v0)]
    fn get_generic_two(self: @ContractState) -> GenericTwo<felt252, u256> {
        GenericTwo { a: 1, b: 2_u256, c: 3, }
    }

    #[external(v0)]
    fn set_generic_two(ref self: ContractState, generic: GenericTwo<u64, u128>) {}

    #[external(v0)]
    fn get_generic_of_generic(self: @ContractState) -> GenericOfGeneric<felt252> {
        GenericOfGeneric { a: GenericOne { a: 0xff, b: 0x1234, c: 1_u256, }, }
    }

    #[external(v0)]
    fn set_generic_of_generics(ref self: ContractState, generic: GenericOfGeneric<u64>) {}

    #[external(v0)]
    fn get_span_generic(self: @ContractState) -> Span<GenericOfGeneric<felt252>> {
        array![
            GenericOfGeneric { a: GenericOne { a: 0xff, b: 0x1234, c: 1_u256, }, }
        ].span()
    }

    #[external(v0)]
    fn set_span_generic(ref self: ContractState, value: Span<GenericOfGeneric<u256>>) {
    }

    #[external(v0)]
    fn get_tuple_generic(self: @ContractState) -> (GenericOne<u256>, GenericOfGeneric<felt252>) {
        (
            GenericOne { a: 0x1, b: 0x2, c: 0x3_u256, },
            GenericOfGeneric { a: GenericOne { a: 0xff, b: 0x1234, c: 1_u256, }, }
        )
    }

    #[external(v0)]
    fn set_tuple_generic(ref self: ContractState, value: (GenericOne<u256>, GenericOfGeneric<felt252>)) {
    }

    #[external(v0)]
    fn get_tuple_of_array_generic(self: @ContractState) -> (Span<GenericOne<u64>>, Span<GenericOfGeneric<felt252>>) {
        (
            array![GenericOne { a: 0x1, b: 0x2, c: 0x3_u256, },].span(),
            array![GenericOfGeneric { a: GenericOne { a: 0xff, b: 0x1234, c: 1_u256, }, },].span(),
        )
    }

    #[external(v0)]
    fn set_tuple_of_array_generic(ref self: ContractState, value: (Span<GenericOne<felt252>>, Span<GenericOfGeneric<u256>>)) {
    }

}
