use crate::{CairoType, Result};
use alloc::{vec, vec::Vec};
use starknet::core::types::FieldElement;

// TODO: add a macro to pre-generate the tuples for more lengths?

impl<T, RT, U, RU> CairoType for (T, U)
where
    T: CairoType<RustType = RT>,
    U: CairoType<RustType = RU>,
{
    type RustType = (RT, RU);

    const SERIALIZED_SIZE: Option<usize> = None;

    #[inline]
    fn serialized_size(rust: &Self::RustType) -> usize {
        T::serialized_size(&rust.0) + U::serialized_size(&rust.1)
    }

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out: Vec<FieldElement> = vec![];
        out.extend(T::serialize(&rust.0));
        out.extend(U::serialize(&rust.1));
        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let mut offset = offset;

        let rust0: RT = T::deserialize(felts, offset)?;
        offset += T::serialized_size(&rust0);

        let rust1: RU = U::deserialize(felts, offset)?;

        Ok((rust0, rust1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_tuple2() {
        let v = (FieldElement::ONE, 128_u32);
        let felts = <(FieldElement, u32)>::serialize(&v);
        assert_eq!(felts.len(), 2);
        assert_eq!(felts[0], FieldElement::ONE);
        assert_eq!(felts[1], FieldElement::from(128_u32));
    }

    #[test]
    fn test_deserialize_tuple2() {
        let felts = vec![FieldElement::THREE, 99_u32.into()];
        let vals = <(FieldElement, u32)>::deserialize(&felts, 0).unwrap();
        assert_eq!(vals.0, FieldElement::THREE);
        assert_eq!(vals.1, 99_u32);
    }

    #[test]
    fn test_serialize_tuple2_array() {
        let v = (vec![FieldElement::ONE], 128_u32);
        let felts = <(Vec<FieldElement>, u32)>::serialize(&v);
        assert_eq!(felts.len(), 3);
        assert_eq!(felts[0], FieldElement::ONE);
        assert_eq!(felts[1], FieldElement::ONE);
        assert_eq!(felts[2], FieldElement::from(128_u32));
    }

    #[test]
    fn test_deserialize_tuple2_array() {
        let felts = vec![FieldElement::ONE, FieldElement::ONE, 99_u32.into()];
        let vals = <(Vec<FieldElement>, u32)>::deserialize(&felts, 0).unwrap();
        assert_eq!(vals.0, vec![FieldElement::ONE]);
        assert_eq!(vals.1, 99_u32);
    }
}
