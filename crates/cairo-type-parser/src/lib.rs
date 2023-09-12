pub mod abi_type;

use starknet::core::types::contract::{EventFieldKind, StateMutability};

use abi_type::AbiType;

#[derive(Debug)]
pub enum CairoAbiEntry {
    Array(Vec<AbiType>),
    Struct(CairoStruct),
    Enum(CairoEnum),
    Function(CairoFunction),
    Event(CairoEvent),
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

// Event
#[derive(Debug)]
pub enum CairoEvent {
    /// Cairo 2.x ABI event entry
    Typed(CairoTypedAbiEvent),
    /// Cairo 1.x ABI event entry
    Untyped(CairoUntypedAbiEvent),
}

#[derive(Debug)]
pub struct CairoUntypedAbiEvent {
    pub name: AbiType,
    pub inputs: Vec<(String, AbiType)>,
}

#[derive(Debug)]
pub enum CairoTypedAbiEvent {
    Struct(CairoAbiEventStruct),
    Enum(CairoAbiEventEnum),
}

#[derive(Debug)]
pub struct CairoAbiEventStruct {
    pub name: AbiType,
    pub members: Vec<(String, AbiType, EventFieldKind)>,
}

#[derive(Debug)]
pub struct CairoAbiEventEnum {
    pub name: AbiType,
    pub variants: Vec<(String, AbiType, EventFieldKind)>,
}
