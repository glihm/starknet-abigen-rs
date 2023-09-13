use cairo_types::*;
use starknet::core::types::*;

// Write enums.
struct MyTuple<T, U>(T, U);

impl<T, U, RT> CairoType for MyTuple<T, U>
where
    T: CairoType<RustType = RT>,
    U: CairoType<RustType = RT>,
{
    type RustType = MyTuple<RT, RT>;

    const SERIALIZED_SIZE: Option<usize> = Some(1);

    const DYNAMIC: bool = Self::SERIALIZED_SIZE.is_none();

    fn serialized_size(_rust: &Self::RustType) -> usize {
        Self::SERIALIZED_SIZE.unwrap()
    }

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out = vec![];

        out.extend(T::serialize(&rust.0));
        out.extend(U::serialize(&rust.1));

        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let mut offset = offset;

        let t = T::deserialize(felts, offset)?;
        offset += T::serialized_size(&t);
        let u = U::deserialize(felts, offset)?;
        offset += U::serialized_size(&u);
        Ok(MyTuple(t, u))
    }
}

pub struct ContractTupleGeneric {}

impl ContractTupleGeneric {
    //
    pub async fn call_example1<T1: CairoType<RustType = T1>, O1: CairoType<RustType = O1>>(
        &self,
        a: (T1, O1),
    ) -> cairo_types::Result<O1> {
        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(<(T1, O1)>::serialize(&a));

        // Deserialization of the data.
        Ok(O1::deserialize(&calldata, 0)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_example() {
        let contract = ContractTupleGeneric {};
        let v = (vec![FieldElement::ONE], 128_u32);
        let result1 = contract.call_example1(v).await.unwrap();
        assert_eq!(result1, 1_u32);
    }
}
