use cairo_types::*;
use starknet::core::types::*;

// Write enums.
struct MyTuple<FieldElement, u32>(FieldElement, u32);

impl CairoType for MyTuple<FieldElement, u32> {
    type RustType = Self;

    const SERIALIZED_SIZE: Option<usize> = Some(1);

    const DYNAMIC: bool = Self::SERIALIZED_SIZE.is_none();

    fn serialized_size(_rust: &Self::RustType) -> usize {
        Self::SERIALIZED_SIZE.unwrap()
    }

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out = vec![];

        out.extend(FieldElement::serialize(&rust.0));
        out.extend(u32::serialize(&rust.1));

        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let mut offset = offset;

        let t = FieldElement::deserialize(felts, offset)?;
        offset += FieldElement::serialized_size(&t);
        let u = u32::deserialize(felts, offset)?;
        offset += u32::serialized_size(&u);
        Ok(MyTuple(t, u))
    }
}

pub struct ContractTupleGeneric {}

impl ContractTupleGeneric {
    //
    pub async fn call_example1(&self, a: (FieldElement, u32)) -> cairo_types::Result<u32> {
        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(<(FieldElement, u32)>::serialize(&a));

        // Deserialization of the data.
        Ok(u32::deserialize(&calldata, 0)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_example() {
        let contract = ContractTupleGeneric {};
        let v = (FieldElement::ONE, 128_u32);
        let result1 = contract.call_example1(v).await.unwrap();
        assert_eq!(result1, 1_u32);
    }
}
