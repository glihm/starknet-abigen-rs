use cairo_types::*;
use starknet::core::types::*;

// Write enums.
#[derive(Debug, PartialEq)]
pub enum MyEnum {
    Var1(u32),
    Var2(FieldElement),
    Var3(Vec<u8>),
}

impl CairoType for MyEnum {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out = vec![];

        match rust {
            MyEnum::Var1(r) => {
                out.push(FieldElement::ZERO);
                out.extend(u32::serialize(r));
            }
            MyEnum::Var2(r) => {
                out.push(FieldElement::ONE);
                out.extend(FieldElement::serialize(r));
            }
            MyEnum::Var3(r) => {
                out.push(FieldElement::TWO);
                out.extend(Vec::<u8>::serialize(r));
            }
        };

        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let idx = felts[offset];

        if idx == FieldElement::ZERO {
            Ok(MyEnum::Var1(u32::deserialize(felts, offset + 1)?))
        } else if idx == FieldElement::ONE {
            Ok(MyEnum::Var2(FieldElement::deserialize(felts, offset + 1)?))
        } else if idx == FieldElement::THREE {
            Ok(MyEnum::Var3(Vec::<u8>::deserialize(felts, offset + 1)?))
        } else {
            Err(Error::Deserialize(
                "Option is expected 0 or 1 index only".to_string(),
            ))
        }
    }
}

pub struct ContractEnumBasic {}

impl ContractEnumBasic {
    //
    pub async fn call_example(&self, a: MyEnum) -> cairo_types::Result<FieldElement> {
        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(MyEnum::serialize(&a));

        // Deserialization of the data.
        FieldElement::deserialize(&calldata, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_example() {
        let contract = ContractEnumBasic {};
        let my_enum = MyEnum::Var1(1_u32);
        let my_enum2 = MyEnum::Var2(FieldElement::TWO);
        let my_enum3 = MyEnum::Var3(vec![1_u8, 3_u8]);
        let result1 = contract.call_example(my_enum).await.unwrap();
        let result2 = contract.call_example(my_enum2).await.unwrap();
        let result3 = contract.call_example(my_enum3).await.unwrap();
        // Because serialize offset is 0, call_example returns index of Enum
        assert_eq!(result1, FieldElement::ZERO);
        assert_eq!(result2, FieldElement::ONE);
        assert_eq!(result3, FieldElement::TWO);
    }
}
