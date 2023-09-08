use crate::ty::CairoType;
use crate::Result;
use alloc::{format, vec, vec::Vec};
use starknet::core::types::FieldElement;

macro_rules! implement_trait_for_unsigned {
    ($type:ty) => {
        impl CairoType for $type {
            type RustType = Self;

            fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
                vec![FieldElement::from(*rust)]
            }

            fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
                let temp: u128 = felts[offset].try_into().unwrap();
                Ok(temp as $type)
            }
        }
    };
}

implement_trait_for_unsigned!(u8);
implement_trait_for_unsigned!(u16);
implement_trait_for_unsigned!(u32);
implement_trait_for_unsigned!(u64);
implement_trait_for_unsigned!(u128);
implement_trait_for_unsigned!(usize);

impl CairoType for FieldElement {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![*rust]
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        Ok(felts[offset])
    }
}

impl CairoType for bool {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![FieldElement::from(*rust as u32)]
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        if felts[offset] == FieldElement::ONE {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Array<CairoType> - `Array<T>`
impl<T, RT> CairoType for Vec<T>
where
    T: CairoType<RustType = RT>,
{
    type RustType = Vec<RT>;

    const SERIALIZED_SIZE: Option<usize> = None;

    #[inline]
    fn serialized_size(rust: &Self::RustType) -> usize {
        let data = rust;
        // 1 + because the length is always the first felt.
        1 + data.iter().map(T::serialized_size).sum::<usize>()
    }

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out: Vec<FieldElement> = vec![rust.len().into()];
        rust.iter().for_each(|r| out.extend(T::serialize(r)));
        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let len: usize = usize::from_str_radix(format!("{:x}", felts[offset]).as_str(), 16)
            .expect("First felt of an array must fit into usize");

        let mut out: Vec<RT> = vec![];
        let mut offset = offset + 1;

        loop {
            if out.len() == len {
                break;
            }

            let rust: RT = T::deserialize(felts, offset)?;
            offset += T::serialized_size(&rust);
            out.push(rust);
        }

        Ok(out)
    }
}

/// Tuple - `tuple`
/// TODO: write a macro to automatically handle more tuple lengths.
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

// *** THIS SHOULD BE GENERATED. HERE FOR TESTING for array
//     with dynamic size.
//***
#[derive(PartialEq, Debug)]
pub struct U256 {
    low: u128,
    high: u128,
}

impl CairoType for U256 {
    type RustType = Self;

    const SERIALIZED_SIZE: Option<usize> = Some(2);

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![FieldElement::from(rust.low), FieldElement::from(rust.high)]
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let low: &[u8; 16] = &felts[offset].to_bytes_be()[16..].try_into().unwrap();

        let high: &[u8; 16] = &felts[offset + 1].to_bytes_be()[16..].try_into().unwrap();

        Ok(U256 {
            low: u128::from_be_bytes(*low),
            high: u128::from_be_bytes(*high),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_bool() {
        let v = true;
        let felts = bool::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::ONE);

        let v = false;
        let felts = bool::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::ZERO);
    }

    #[test]
    fn test_deserialize_bool() {
        let felts = vec![FieldElement::ZERO, FieldElement::ONE, FieldElement::TWO]; // allocating in the heap.
        assert_eq!(bool::deserialize(&felts, 0).unwrap(), false);
        assert_eq!(bool::deserialize(&felts, 1).unwrap(), true);
        assert_eq!(bool::deserialize(&felts, 2).unwrap(), false);
    }

    #[test]
    fn test_serialize_u8() {
        let v = 12_u8;
        let felts = u8::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(12 as u8));
    }

    #[test]
    fn test_deserialize_u8() {
        let felts = vec![FieldElement::from(12_u8), FieldElement::from(10_u8)];
        assert_eq!(u8::deserialize(&felts, 0).unwrap(), 12);
        assert_eq!(u8::deserialize(&felts, 1).unwrap(), 10);
    }

    #[test]
    fn test_serialize_u32() {
        let v = 123_u32;
        let felts = u32::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(123 as u32));
    }

    #[test]
    fn test_serialize_u16() {
        let v = 12_u16;
        let felts = u16::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(12 as u16));
    }

    #[test]
    fn test_deserialize_u16() {
        let felts = vec![FieldElement::from(12_u16), FieldElement::from(10_u8)];
        assert_eq!(u16::deserialize(&felts, 0).unwrap(), 12);
        assert_eq!(u16::deserialize(&felts, 1).unwrap(), 10);
    }

    #[test]
    fn test_deserialize_u32() {
        let felts = vec![FieldElement::from(123_u32), FieldElement::from(99_u32)];
        assert_eq!(u32::deserialize(&felts, 0).unwrap(), 123);
        assert_eq!(u32::deserialize(&felts, 1).unwrap(), 99);
    }

    #[test]
    fn test_serialize_u64() {
        let v = 123_u64;
        let felts = u64::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(123 as u64));
    }

    #[test]
    fn test_deserialize_u64() {
        let felts = vec![FieldElement::from(123_u64), FieldElement::from(99_u64)];
        assert_eq!(u64::deserialize(&felts, 0).unwrap(), 123);
        assert_eq!(u64::deserialize(&felts, 1).unwrap(), 99);
    }

    #[test]
    fn test_serialize_u128() {
        let v = 123_u128;
        let felts = u128::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(123 as u128));
    }

    #[test]
    fn test_deserialize_u128() {
        let felts = vec![FieldElement::from(123_u128), FieldElement::from(99_u128)];
        assert_eq!(u128::deserialize(&felts, 0).unwrap(), 123);
        assert_eq!(u128::deserialize(&felts, 1).unwrap(), 99);
    }

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

    #[test]
    fn test_serialize_array() {
        let v: Vec<u32> = vec![1, 2, 3];
        let felts = Vec::<u32>::serialize(&v);
        assert_eq!(felts.len(), 4);
        assert_eq!(felts[0], FieldElement::from(3_u32));
        assert_eq!(felts[1], FieldElement::ONE);
        assert_eq!(felts[2], FieldElement::TWO);
        assert_eq!(felts[3], FieldElement::THREE);

        let v: Vec<U256> = vec![U256 { low: 1, high: 0 }, U256 { low: 2, high: 0 }];

        let felts = Vec::<U256>::serialize(&v);
        assert_eq!(felts.len(), 5);
        assert_eq!(felts[1], FieldElement::ONE);
        assert_eq!(felts[2], FieldElement::ZERO);
        assert_eq!(felts[3], FieldElement::TWO);
        assert_eq!(felts[4], FieldElement::ZERO);
    }

    #[test]
    fn test_deserialize_array() {
        let felts: Vec<FieldElement> = vec![
            FieldElement::from(2_u32),
            FieldElement::from(123_u32),
            FieldElement::from(9988_u32),
        ];

        let vals = Vec::<u32>::deserialize(&felts, 0).unwrap();
        assert_eq!(vals.len(), 2);
        assert_eq!(vals[0], 123_u32);
        assert_eq!(vals[1], 9988_u32);

        let felts: Vec<FieldElement> = vec![
            FieldElement::from(2_u32),
            FieldElement::from(1_u32),
            FieldElement::from(0_u32),
            FieldElement::from(2_u32),
            FieldElement::from(0_u32),
        ];

        let vals = Vec::<U256>::deserialize(&felts, 0).unwrap();
        assert_eq!(vals.len(), 2);
        assert_eq!(vals[0], U256 { low: 1, high: 0 });
        assert_eq!(vals[1], U256 { low: 2, high: 0 });
    }

    #[test]
    fn test_serialize_array_nested() {
        let v: Vec<Vec<u32>> = vec![vec![1, 2], vec![3]];
        let felts = Vec::<Vec<u32>>::serialize(&v);
        assert_eq!(felts.len(), 6);
        assert_eq!(felts[0], FieldElement::TWO);
        assert_eq!(felts[1], FieldElement::TWO);
        assert_eq!(felts[2], FieldElement::ONE);
        assert_eq!(felts[3], FieldElement::TWO);
        assert_eq!(felts[4], FieldElement::ONE);
        assert_eq!(felts[5], FieldElement::THREE);
    }

    #[test]
    fn test_deserialize_array_nested() {
        let felts: Vec<FieldElement> = vec![
            FieldElement::TWO,
            FieldElement::TWO,
            FieldElement::ONE,
            FieldElement::TWO,
            FieldElement::ONE,
            FieldElement::THREE,
        ];

        let vals = Vec::<Vec<u32>>::deserialize(&felts, 0).unwrap();
        assert_eq!(vals.len(), 2);
        assert_eq!(vals[0], vec![1, 2]);
        assert_eq!(vals[1], vec![3]);
    }

    #[test]
    fn test_serialize_array_tuple() {
        let v: Vec<(u32, FieldElement)> = vec![(12, FieldElement::TWO)];
        let felts = Vec::<(u32, FieldElement)>::serialize(&v);
        assert_eq!(felts.len(), 3);
        assert_eq!(felts[0], FieldElement::from(1_u32));
        assert_eq!(felts[1], FieldElement::from(12_u32));
        assert_eq!(felts[2], FieldElement::TWO);
    }

    #[test]
    fn test_deserialize_array_tuple() {
        let felts: Vec<FieldElement> = vec![
            FieldElement::from(1_u32),
            FieldElement::from(12_u32),
            FieldElement::TWO,
        ];

        let vals = Vec::<(u32, FieldElement)>::deserialize(&felts, 0).unwrap();
        assert_eq!(vals.len(), 1);
        assert_eq!(vals[0], (12, FieldElement::TWO));
    }
}
