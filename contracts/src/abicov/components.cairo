//! A contract with components.
#[starknet::interface]
trait ISimple<TContractState> {
    fn read_data(self: @TContractState) -> felt252;
    fn write_data(ref self: TContractState, data: felt252);
}

#[starknet::component]
mod simple_component {
    #[storage]
    struct Storage {
        data: felt252,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        Written: Written
    }

    #[derive(Drop, starknet::Event)]
    struct Written {
        before: felt252,
        after: felt252,
    }

    #[embeddable_as(Simple)]
    impl SimpleImpl<
        TContractState, +HasComponent<TContractState>
    > of super::ISimple<ComponentState<TContractState>> {
        fn read_data(self: @ComponentState<TContractState>) -> felt252 {
            self.data.read()
        }

        fn write_data(ref self: ComponentState<TContractState>, data: felt252) {
            self.data.write(data);
        }
    }
}

#[starknet::contract]
mod components {
    use super::simple_component;

    component!(path: simple_component, storage: simple, event: SimpleEvent);

    #[abi(embed_v0)]
    impl SimpleImpl = simple_component::Simple<ContractState>;

    #[storage]
    struct Storage {
        value: felt252,
        #[substorage(v0)]
        simple: simple_component::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        OutterEvent: OutterEvent,
        // The flat attribute is just changing the `kind` in the ABI.
        // Instead of `nested` it's `flat`.
        #[flat]
        SimpleEvent: simple_component::Event,
    }

    #[derive(Drop, starknet::Event)]
    struct OutterEvent {}

    #[external(v0)]
    fn foo(ref self: ContractState) {
        self.simple.write_data(0xaa);
        self.value.write(0xff);
    }
}
