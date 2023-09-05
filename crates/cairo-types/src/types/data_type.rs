use crate::{Result};
use alloc::{vec::Vec, vec};
use starknet::core::types::FieldElement;
use crate::ty::CairoType;
use core::marker::PhantomData;

/// FieldElement - `felt252`
impl CairoType for FieldElement {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![*rust]
    }

    fn deserialize(felts: &[FieldElement], offset: &mut usize) -> Result<Self::RustType> {
        Ok(felts[*offset])
    }
}

/// Bool - `bool`
pub struct Bool;

impl CairoType for Bool {
    type RustType = bool;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![FieldElement::from(*rust as u32)]
    }

    fn deserialize(felts: &[FieldElement], offset: &mut usize) -> Result<Self::RustType> {
        if felts[*offset] == FieldElement::ONE {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// U32 - `u32`
pub struct U32;

impl CairoType for U32 {
    type RustType = u32;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![FieldElement::from(*rust)]
    }

    fn deserialize(felts: &[FieldElement], offset: &mut usize) -> Result<Self::RustType> {
        // TODO: that's ugly or fine? We do know felt is always &[u8; 32]
        // byte array.
        let bytes: &[u8; 4] = &felts[*offset].to_bytes_be()[28..]
            .try_into()
            .unwrap();

        Ok(u32::from_be_bytes(*bytes))
    }
}

/// U128 - `u128`
pub struct U128;

impl CairoType for U128 {
    type RustType = u128;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![FieldElement::from(*rust)]
    }

    fn deserialize(felts: &[FieldElement], offset: &mut usize) -> Result<Self::RustType> {
        // TODO: that's ugly or fine? We do know felt is always &[u8; 32]
        // byte array.
        let bytes: &[u8; 16] = &felts[*offset].to_bytes_be()[16..]
            .try_into()
            .unwrap();

        Ok(u128::from_be_bytes(*bytes))
    }
}

/// Array<FieldElement> - `Array<felt252>`
pub struct Array<T: CairoType>(PhantomData<T>);

impl<T, U> CairoType for Array<T> where T: CairoType<RustType = U> {
    type RustType = Vec<U>;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out: Vec<FieldElement> = vec![rust.len().into()];
        rust.iter().for_each(|r| out.extend(T::serialize(r)));
        out
    }

    fn deserialize(_felts: &[FieldElement], _offset: &mut usize) -> Result<Self::RustType> {
        // TODO: we need the length knowledge here to be able to
        // deserialize a generic array of CairoType elements.
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_bool() {
        let v = true;
        let felts = Bool::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::ONE);

        let v = false;
        let felts = Bool::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::ZERO);
    }

    #[test]
    fn test_deserialize_bool() {
        let felts = vec![FieldElement::ZERO, FieldElement::ONE, FieldElement::TWO]; // allocating in the heap.
        assert_eq!(Bool::deserialize(&felts, &mut 0).unwrap(), false);
        assert_eq!(Bool::deserialize(&felts, &mut 1).unwrap(), true);
        assert_eq!(Bool::deserialize(&felts, &mut 2).unwrap(), false);
    }

    #[test]
    fn test_serialize_u32() {
        let v = 123_u32;
        let felts = U32::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(123 as u32));
    }

    #[test]
    fn test_deserialize_u32() {
        let felts = vec![FieldElement::from(123_u32), FieldElement::from(99_u32)];
        assert_eq!(U32::deserialize(&felts, &mut 0).unwrap(), 123);
        assert_eq!(U32::deserialize(&felts, &mut 1).unwrap(), 99);
    }

    #[test]
    fn test_serialize_u128() {
        let v = 123_u128;
        let felts = U128::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(123 as u128));
    }

    #[test]
    fn test_deserialize_u128() {
        let felts = vec![FieldElement::from(123_u128), FieldElement::from(99_u128)];
        assert_eq!(U128::deserialize(&felts, &mut 0).unwrap(), 123);
        assert_eq!(U128::deserialize(&felts, &mut 1).unwrap(), 99);
    }

    #[test]
    fn test_serialize_array() {
        let v: Vec<u32> = vec![1, 2, 3];
        let felts = Array::<U32>::serialize(&v);
        assert_eq!(felts.len(), 4);
        assert_eq!(felts[0], FieldElement::from(3_u32));
        assert_eq!(felts[1], FieldElement::ONE);
        assert_eq!(felts[2], FieldElement::TWO);
        assert_eq!(felts[3], FieldElement::THREE);
    }

}
