use cairo_types::*;
use starknet::core::types::*;
use starknet::macros::felt;

// Write enums.
#[derive(Debug, PartialEq)]
pub enum MyEnum<A> {
    Var1(A),
    Var2(FieldElement),
    Var3(Vec<A>),
}

impl<T, RT> CairoType for MyEnum<T>
where
    T: CairoType<RustType = RT>,
{
    type RustType = MyEnum<RT>;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out = vec![];

        match rust {
            MyEnum::Var1(r) => {
                out.push(FieldElement::ZERO);
                out.extend(T::serialize(r));
            }
            MyEnum::Var2(r) => {
                out.push(FieldElement::ONE);
                out.extend(FieldElement::serialize(r));
            }
            MyEnum::Var3(r) => {
                out.push(FieldElement::TWO);
                out.extend(Vec::<T>::serialize(r));
            }
        };

        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let idx = felts[offset];

        if idx == FieldElement::ZERO {
            Ok(MyEnum::Var1(T::deserialize(felts, offset + 1)?))
        } else if idx == FieldElement::ONE {
            Ok(MyEnum::Var2(FieldElement::deserialize(felts, offset + 1)?))
        } else if idx == FieldElement::THREE {
            Ok(MyEnum::Var3(Vec::<T>::deserialize(felts, offset + 1)?))
        } else {
            Err(Error::Deserialize(
                "Option is expected 0 or 1 index only".to_string(),
            ))
        }
    }
}

pub struct ContractEnumGeneric {}

impl ContractEnumGeneric {
    //
    pub async fn call_example<T1: CairoType<RustType = T1>>(
        &self,
        a: MyEnum<T1>,
    ) -> cairo_types::Result<FieldElement> {
        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(MyEnum::<T1>::serialize(&a));

        // Deserialization of the data.
        FieldElement::deserialize(&calldata, 0)
    }

    //
    pub async fn call_example2<T1: CairoType<RustType = T1>, O1: CairoType<RustType = O1>>(
        &self,
        a: MyEnum<T1>,
        b: MyEnum<O1>,
    ) -> cairo_types::Result<MyEnum<O1>> {
        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(MyEnum::<T1>::serialize(&a));
        calldata.extend(MyEnum::<O1>::serialize(&b));

        // Deserialization of the data.
        MyEnum::<O1>::deserialize(&calldata, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_example() {
        let contract = ContractEnumGeneric {};
        let my_enum = MyEnum::Var1(FieldElement::ONE);
        let my_enum2 = MyEnum::<FieldElement>::Var2(FieldElement::TWO);
        let my_enum3 = MyEnum::Var3(vec![1_u32, 2_32]);
        let result1 = contract.call_example(my_enum).await.unwrap();
        let result2 = contract.call_example(my_enum2).await.unwrap();
        let result3 = contract.call_example(my_enum3).await.unwrap();
        // Because serialize offset is 0, call_example returns index of Enum
        assert_eq!(result1, FieldElement::ZERO);
        assert_eq!(result2, FieldElement::ONE);
        assert_eq!(result3, FieldElement::TWO);
    }

    #[tokio::test]
    async fn test_call_example2() {
        let contract = ContractEnumGeneric {};
        let my_enum = MyEnum::Var1(FieldElement::ONE);
        let my_enum3 = MyEnum::Var3(vec![1_u32, 2_32]);
        let result1 = contract.call_example2(my_enum, my_enum3).await.unwrap();
        assert_eq!(result1, MyEnum::Var1(1_u32));
    }
}
