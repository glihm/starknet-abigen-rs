use starknet::core::types::contract::{AbiNamedMember, StateMutability};

use super::abi_types::AbiTypeAny;

#[derive(Debug, Clone)]
pub struct CairoFunction {
    pub name: String,
    pub state_mutability: StateMutability,
    pub inputs: Vec<(String, AbiTypeAny)>,
    // cairo 0 can have multiple outputs.
    pub outputs: Vec<(String, AbiTypeAny)>,
}

impl CairoFunction {
    /// Initializes a new instance from the abi name and it's members.
    pub fn new(
        abi_name: &str,
        state_mutability: StateMutability,
        inputs: &[AbiNamedMember],
        outputs: &[AbiNamedMember],
    ) -> CairoFunction {
        let name = abi_name.to_string();

        let outputs = outputs
            .iter()
            .map(|o| (o.name.clone(), AbiTypeAny::from_string(&o.r#type)))
            .collect();

        let inputs = inputs
            .iter()
            .map(|i| (i.name.clone(), AbiTypeAny::from_string(&i.r#type)))
            .collect();

        CairoFunction {
            name,
            state_mutability,
            inputs,
            outputs,
        }
    }
}
