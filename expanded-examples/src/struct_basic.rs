use cairo_types::*;
use starknet::core::types::*;

// Write structs
#[derive(Debug, PartialEq)]
pub struct MyStruct {
    field1: u32,
    field2: FieldElement,
    field3: Vec<u8>,
}

impl CairoType for MyStruct {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out = vec![];

        out.extend(u32::serialize(&rust.field1));
        out.extend(FieldElement::serialize(&rust.field2));
        out.extend(Vec::<u8>::serialize(&rust.field3));

        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let mut offset = offset;

        let field1 = u32::deserialize(felts, offset)?;
        offset += u32::serialized_size(&field1);
        let field2 = FieldElement::deserialize(felts, offset)?;
        offset += FieldElement::serialized_size(&field2);
        let field3 = Vec::<u8>::deserialize(felts, offset)?;
        offset += Vec::<u8>::serialized_size(&field3);

        Ok(MyStruct {
            field1,
            field2,
            field3,
        })
    }
}

pub struct ContractStructBasic {}

impl ContractStructBasic {
    //
    pub async fn call_example(&self, a: MyStruct) -> cairo_types::Result<FieldElement> {
        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(MyStruct::serialize(&a));

        // Make the call, which always return an array of felts.

        // Deserialization of the data.
        FieldElement::deserialize(&calldata, 0)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[tokio::test]
    async fn test_call_example() {
        let contract = ContractStructBasic {};
        let my_struct = MyStruct {
            field1: 1_u32,
            field2: FieldElement::ONE,
            field3: vec![1_u8, 3_u8],
        };

        let result1 = contract.call_example(my_struct).await.unwrap();

        assert_eq!(result1, FieldElement::ONE);
    }
}
