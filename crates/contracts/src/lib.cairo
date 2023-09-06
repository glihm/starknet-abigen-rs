#[starknet::contract]
mod c1 {
    use serde::Serde;
    use array::ArrayTrait;

    #[derive(Serde, Drop)]
    enum TestEnum<T> {
        V1: T, // 0
        V2: u128, // 1
        V3: Array<felt252>, // 2
        V4: (), // 3
    }
    
    #[derive(Serde, Drop)]
    struct TypesInTypes<T> {
        a: Array<Array<felt252>>,
        b: T,
    }

    #[storage]
    struct Storage {
        
    }

    #[derive(Serde, Drop)]
    struct InnerOne {
        a: u256,
        b: Array<felt252>,
    }

    #[derive(Serde, Drop)]
    struct ComplexOne {
        pg: PG,
        inner: InnerOne,
    }

    #[derive(Serde, Drop)]
    struct PG {
        v1: felt252,
        v2: u128,
    }

    #[external(v0)]
    fn hello_world(self: @ContractState, value: felt252) -> PG {
        PG {
            v1: value,
            v2: 11_u128
        }
    }

    #[external(v0)]
    fn call_test(ref self: ContractState, pg: PG, inner: InnerOne) -> ComplexOne {
        ComplexOne {
            pg,
            inner
        }
    }

    #[external(v0)]
    fn call_bla(ref self: ContractState, a: TypesInTypes<u256>) -> TypesInTypes<u256> {
        a
    }

    #[external(v0)]
    fn call_bou(ref self: ContractState) -> (Span<felt252>, felt252) {
        (array![1,2].span(), 2)
    }

    #[external(v0)]
    fn call_bii(ref self: ContractState) -> (Span<(felt252, PG)>, felt252) {
        let item: (felt252, PG) = (1, PG { v1: 1, v2: 2_u128 });
        (array![item].span(), 2)
    }

    #[external(v0)]
    fn call_baba(ref self: ContractState) -> TestEnum<felt252> {
        TestEnum::V1(2)
    }

    #[external(v0)]
    fn call_baba2(ref self: ContractState) -> TestEnum<u256> {
        TestEnum::V1(u256 { low: 1_u128, high: 0_u128 })
    }

    #[external(v0)]
    fn call_pia(ref self: ContractState) -> TestEnum<felt252> {
        TestEnum::V3(array![1,2,3])
    }

    #[external(v0)]
    fn call_bibi(ref self: ContractState) -> Option<felt252> {
        Option::Some(0xff)
    }

    #[external(v0)]
    fn call_bobo(ref self: ContractState) -> Option<felt252> {
        Option::None
    }

    #[external(v0)]
    fn call_bok(ref self: ContractState) -> (felt252, felt252) {
        (1, 2)
    }
}
