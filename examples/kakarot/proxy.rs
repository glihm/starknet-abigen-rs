use starknet::{
    accounts::{Account, ConnectedAccount},
    core::types::FieldElement,
};
use starknet_abigen::macros::abigen_legacy;

abigen_legacy!(KkrtProxy, "./examples/kakarot/abi/proxy.json");
