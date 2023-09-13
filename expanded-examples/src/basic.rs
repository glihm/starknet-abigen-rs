use cairo_types::*;
use starknet::macros::felt;
use starknet::core::types::*;

// Write structs / enums.

pub struct ContractA {

}

impl ContractA {

    //
    pub async fn call_example(
        &self,
        a: FieldElement
    ) -> cairo_types::Result<FieldElement> {

        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(FieldElement::serialize(&a));

        // Make the call, which always return an array of felts.
        let result = vec![felt!("0")];

        // Deserialization of the data.
        FieldElement::deserialize(&result, 0)
    }

    //
    pub async fn invoke_example(
        &self,
        v: FieldElement,
    ) -> anyhow::Result<FieldElement> {
        // Serialization of inputs.
        let mut calldata = Vec::new();
        calldata.extend(FieldElement::serialize(&v));

        // ...

        // The return type is always a single felt (the transaction hash).
        Ok(FieldElement::ONE)
    }
}
