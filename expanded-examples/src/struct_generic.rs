use cairo_types::*;
use starknet::core::types::*;

// Write structs
#[derive(Debug, PartialEq)]
pub struct MyStruct<A> {
    field1: A,
    field2: FieldElement,
    field3: Vec<A>,
}

impl<T, RT> CairoType for MyStruct<T>
where
    T: CairoType<RustType = RT>,
{
    type RustType = MyStruct<RT>;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out = vec![];

        out.extend(T::serialize(&rust.field1));
        out.extend(FieldElement::serialize(&rust.field2));
        out.extend(Vec::<T>::serialize(&rust.field3));

        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let mut offset = offset;

        let field1 = T::deserialize(felts, offset)?;
        offset += T::serialized_size(&field1);
        let field2 = FieldElement::deserialize(felts, offset)?;
        offset += FieldElement::serialized_size(&field2);
        let field3 = Vec::<T>::deserialize(felts, offset)?;
        offset += Vec::<T>::serialized_size(&field3);

        Ok(MyStruct {
            field1,
            field2,
            field3,
        })
    }
}

pub struct ContractStructGeneric {}

impl ContractStructGeneric {
    //
    pub async fn call_example<T1: CairoType<RustType = T1>>(
        &self,
        a: MyStruct<T1>,
    ) -> cairo_types::Result<FieldElement> {
        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(MyStruct::<T1>::serialize(&a));

        // Make the call, which always return an array of felts.

        // Deserialization of the data.
        FieldElement::deserialize(&calldata, 0)
    }

    //
    pub async fn call_example2<T1: CairoType<RustType = T1>, O1: CairoType<RustType = O1>>(
        &self,
        a: MyStruct<T1>,
        b: MyStruct<O1>,
    ) -> cairo_types::Result<MyStruct<O1>> {
        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(MyStruct::<T1>::serialize(&a));
        calldata.extend(MyStruct::<O1>::serialize(&b));

        // Deserialization of the data.
        MyStruct::<O1>::deserialize(&calldata, 5)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[tokio::test]
    async fn test_call_example() {
        let contract = ContractStructGeneric {};
        let my_struct = MyStruct {
            field1: 1_u32,
            field2: FieldElement::ONE,
            field3: vec![1_u32, 3_u32],
        };

        let result1 = contract.call_example(my_struct).await.unwrap();

        assert_eq!(result1, FieldElement::ONE);
    }

    #[tokio::test]
    async fn test_call_example2() {
        let contract = ContractStructGeneric {};
        let my_struct1 = MyStruct {
            field1: 1_u32,
            field2: FieldElement::ONE,
            field3: vec![1_u32, 3_u32],
        };

        let my_struct2 = MyStruct {
            field1: vec![FieldElement::ONE],
            field2: FieldElement::ONE,
            field3: vec![vec![FieldElement::ONE], vec![FieldElement::ONE]],
        };

        let result1 = contract
            .call_example2(my_struct1, my_struct2)
            .await
            .unwrap();

        assert_eq!(
            result1,
            MyStruct {
                field1: vec![FieldElement::ONE],
                field2: FieldElement::ONE,
                field3: vec![vec![FieldElement::ONE], vec![FieldElement::ONE]],
            }
        );
    }
}
