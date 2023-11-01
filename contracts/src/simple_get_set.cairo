#[starknet::contract]
mod simple_get_set {
    #[storage]
    struct Storage {
        a: felt252,
        b: u256,
    }
    
    #[external(v0)]
    fn get_a(self: @ContractState) -> felt252 {
        self.a.read()
    }

    #[external(v0)]
    fn set_a(ref self: ContractState, a: felt252) {
        self.a.write(a);
    }

    #[external(v0)]
    fn get_b(self: @ContractState) -> u256 {
        self.b.read()
    }

    #[external(v0)]
    fn set_b(ref self: ContractState, b: u256) {
        self.b.write(b);
    }
}
