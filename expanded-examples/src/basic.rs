use cairo_types::*;
use starknet::core::types::*;

pub struct ContractA {}

impl ContractA {
    //
    pub fn call_example(&self, a: FieldElement) -> cairo_types::Result<FieldElement> {
        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(FieldElement::serialize(&a));

        // Deserialization of the data.
        FieldElement::deserialize(&calldata, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_example() {
        let contract = ContractA {};
        let input = FieldElement::ONE;
        let result = contract.call_example(input).unwrap();
        assert_eq!(result, FieldElement::ONE);
    }
}
