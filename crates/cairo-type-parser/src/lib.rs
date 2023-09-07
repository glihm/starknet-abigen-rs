pub mod abi_type;

use proc_macro2::TokenStream as TokenStream2;
use std::collections::HashMap;
use syn::Type;

use starknet::core::types::contract::StateMutability;

use abi_type::AbiType;

#[derive(Debug)]
pub enum CairoAbiEntry {
    Struct(CairoStruct),
    Enum(CairoEnum),
    Function(CairoFunction),
}

#[derive(Debug)]
pub struct CairoStruct {
    pub name: AbiType,
    pub members: Vec<(String, AbiType)>,
}

#[derive(Debug)]
pub struct CairoEnum {
    pub name: AbiType,
    pub variants: Vec<(String, AbiType)>,
}

#[derive(Debug)]
pub struct CairoFunction {
    // TODO: perhaps the name can be a regular string.
    pub name: AbiType,
    pub state_mutability: StateMutability,
    pub inputs: Vec<(String, AbiType)>,
    // For now, only one output type is supported (or none).
    pub output: Option<AbiType>,
}
