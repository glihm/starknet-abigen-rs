pub mod abi_type;

use proc_macro2::TokenStream as TokenStream2;
use std::collections::HashMap;
use syn::Type;

use abi_type::AbiType;

pub enum CairoFunctionStateMutability {
    View,
    External,
}

pub enum CairoAbiEntry {
    Struct(CairoStruct),
    Enum(CairoEnum),
    Function(CairoFunction),
}

pub struct CairoStruct {
    pub name: AbiType,
    pub members: Vec<(String, AbiType)>,
}

pub struct CairoEnum {
    pub name: AbiType,
    pub variants: Vec<(String, AbiType)>,
}

pub struct CairoFunction {
    pub name: AbiType,
    pub state_mutability: CairoFunctionStateMutability,
    pub inputs: Vec<(String, AbiType)>,
    pub outputs: Vec<AbiType>,
}
