use std::collections::HashMap;

use super::{CairoAny, CairoStruct, CairoEnum};
use super::abi_types::{AbiType, AbiTypeAny};
use starknet::core::types::contract::{AbiNamedMember, AbiOutput, StateMutability};

#[derive(Debug, Clone)]
pub struct CairoFunction {
    pub name: String,
    pub state_mutability: StateMutability,
    pub inputs: Vec<(String, CairoAny)>,
    // For now, only one output type is supported (or none).
    // TODO: investigate the cases where more than one output is
    // present in the ABI.
    pub output: Option<CairoAny>,
}

impl CairoFunction {
    pub fn is_generic(&self) -> bool {
        if let Some(o) = &self.output {
            return o.is_generic();
        };

        for (_, cairo_type) in &self.inputs {
            if cairo_type.is_generic() {
                return true;
            }
        }

        false
    }

    pub fn get_gentys(&self) -> Vec<String> {
        let gentys = vec![];
        // As we can have several generics inside arguments or output,
        // we need to prefix them with the argument name to avoid conflicts.

        gentys
    }

    /// Initializes a new instance from the abi name and it's members.
    pub fn new(
        abi_name: &str,
        state_mutability: StateMutability,
        inputs: &Vec<AbiNamedMember>,
        outputs: &Vec<AbiOutput>,
    ) -> CairoFunction {
        let name = abi_name.to_string();
        let inputs = vec![];
        let output = None;

        CairoFunction {
            name,
            state_mutability,
            inputs,
            output,
        }
    }

}
