use super::{CairoEnum, CairoStruct};
use crate::abi_types::{AbiType, AbiTypeAny};
use starknet::core::types::contract::{AbiNamedMember, AbiOutput, StateMutability};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CairoFunction {
    pub name: String,
    pub state_mutability: StateMutability,
    pub inputs: Vec<(String, AbiTypeAny)>,
    // For now, only one output type is supported (or none).
    // TODO: investigate the cases where more than one output is
    // present in the ABI.
    pub output: Option<AbiTypeAny>,
}

impl CairoFunction {
    pub fn is_generic(&self) -> bool {
        if let Some(o) = &self.output {
            return o.is_generic();
        };

        for (_, abi_type) in &self.inputs {
            if abi_type.is_generic() {
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
        structs: &HashMap<String, CairoStruct>,
        enums: &HashMap<String, CairoEnum>,
    ) -> CairoFunction {
        println!("\nSTRUCTS\n{:?}", structs);
        println!("\nENUMS\n{:?}", enums);

        let name = abi_name.to_string();

        let output = if !outputs.is_empty() {
            // For now, only first output is considered.
            // TODO: investigate when we can have several outputs.
            let abi_type = AbiTypeAny::from_string(&outputs[0].r#type);
            // let abi_type = match abi_type {
            //     AbiTypeAny::Generic(ref g) => {
            //         println!("GENERIC! ABITYPE: {:?}", g.get_rust_generic_def("_a"));
            //         if let Some(ref e) = enums.get(&g.get_cairo_type_name_only()) {
            //             e.abi.clone()
            //         } else if let Some(ref s) = structs.get(&g.get_cairo_type_name_only()) {
            //             s.abi.clone()
            //         } else {
            //             abi_type
            //         }
            //     }
            //     _ => abi_type
            // };

            Some(abi_type)
        } else {
            None
        };

        let inputs = inputs
            .iter()
            .map(|i| {
                let abi_type = AbiTypeAny::from_string(&i.r#type);
                // let abi_type = match abi_type {
                //     AbiTypeAny::Generic(ref g) => {
                //         if let Some(ref e) = enums.get(&g.get_cairo_type_name_only()) {
                //             e.abi.clone()
                //         } else if let Some(ref s) = structs.get(&g.get_cairo_type_name_only()) {
                //             s.abi.clone()
                //         } else {
                //             abi_type
                //         }
                //     }
                //     _ => abi_type
                // };
                (i.name.clone(), abi_type)
            })
            .collect();

        CairoFunction {
            name,
            state_mutability,
            inputs,
            output,
        }
    }

}
