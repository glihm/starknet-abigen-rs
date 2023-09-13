use cairo_types::*;
use starknet::macros::felt;
use starknet::core::types::*;

// Write structs / enums.
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

pub struct ContractEnumGeneric {

}

impl ContractEnumGeneric {

    //
    pub async fn call_example<T1: CairoType<RustType = T1>>(
        &self,
        a: MyEnum<T1>
    ) -> cairo_types::Result<FieldElement> {

        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(MyEnum::<T1>::serialize(&a));

        // Make the call, which always return an array of felts.
        let result = vec![felt!("0")];

        // Deserialization of the data.
        FieldElement::deserialize(&result, 0)
    }

    //
    pub async fn call_example2<T1: CairoType<RustType = T1>, O1: CairoType<RustType = O1>>(
        &self,
        a: MyEnum<T1>
    ) -> cairo_types::Result<MyEnum<O1>> {

        // Serialization of the data.
        let mut calldata = Vec::new();
        calldata.extend(MyEnum::<T1>::serialize(&a));

        // Make the call, which always return an array of felts.
        let result = vec![felt!("0")];

        // Deserialization of the data.
        MyEnum::<O1>::deserialize(&result, 0)
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
