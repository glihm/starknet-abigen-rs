use starknet::core::types::contract::{AbiNamedMember, AbiOutput, StateMutability};

use super::abi_types::AbiTypeAny;

#[derive(Debug, Clone)]
pub struct CairoFunction {
    pub name: String,
    pub state_mutability: StateMutability,
    pub inputs: Vec<(String, AbiTypeAny)>,
    pub outputs: Vec<AbiTypeAny>,
}

impl CairoFunction {
    /// Initializes a new instance from the abi name and it's members.
    pub fn new(
        abi_name: &str,
        state_mutability: StateMutability,
        inputs: &[AbiNamedMember],
        outputs: &[AbiOutput],
    ) -> Self {
        let name = abi_name.to_string();

        let outputs = outputs
            .iter()
            .map(|o| AbiTypeAny::from_string(&o.r#type))
            .collect();

        let inputs = inputs
            .iter()
            .map(|i| (i.name.clone(), AbiTypeAny::from_string(&i.r#type)))
            .collect();

        Self {
            name,
            state_mutability,
            inputs,
            outputs,
        }
    }
}

// We need an other impl of Expandable for this one, hence a new type,
// even if the content of the struct is very similar.
#[derive(Debug, Clone)]
pub struct CairoFunctionLegacy {
    pub name: String,
    pub state_mutability: StateMutability,
    pub inputs: Vec<(String, AbiTypeAny)>,
    pub outputs: Vec<AbiTypeAny>,
}

impl CairoFunctionLegacy {
    /// Initializes a new instance from the abi name and it's members.
    pub fn new(
        abi_name: &str,
        state_mutability: StateMutability,
        inputs: &[AbiNamedMember],
        outputs: &[AbiOutput],
    ) -> Self {
        let name = abi_name.to_string();

        let outputs = outputs
            .iter()
            .map(|o| AbiTypeAny::from_string(&o.r#type))
            .collect();

        let inputs = inputs
            .iter()
            .map(|i| (i.name.clone(), AbiTypeAny::from_string(&i.r#type)))
            .collect();

        Self {
            name,
            state_mutability,
            inputs,
            outputs,
        }
    }
}
