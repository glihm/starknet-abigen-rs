use crate::{Result};
use alloc::{vec::Vec};
use starknet::core::types::FieldElement;
use crate::ty::CairoType;
use core::marker::PhantomData;

// AUTO GENERATED code example!
pub struct U256 {
    low: u128,
    high: u128,
}

impl CairoType for U256 {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![
            FieldElement::from(rust.low),
            FieldElement::from(rust.high)
        ]
    }

    fn deserialize(felts: &[FieldElement]) -> Result<Self::RustType> {
        Ok(U256 {
            low: 0,
            high: 0,
        })
    }
}


/// RustOption - Example on how implementing a type that is
/// depending on an other type using T.
pub struct CairoOption<T: CairoType>(PhantomData<T>);

impl<T, U> CairoType for CairoOption<T> where T: CairoType<RustType = U> {
    type RustType = Option<U>;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        match rust {
            Some(v) => {
                let mut felts = vec![FieldElement::ZERO];
                felts.extend(T::serialize(v));
                felts
            }
            None => vec![FieldElement::ONE]
        }
    }

    fn deserialize(_felts: &[FieldElement]) -> Result<Self::RustType> {
        Ok(Option::None)
    }
}
// ********* EXAMPLE ****

/// FieldElement - `felt252`
impl CairoType for FieldElement {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![*rust]
    }

    fn deserialize(felts: &[FieldElement]) -> Result<Self::RustType> {
        Ok(felts[0])
    }
}

/// Bool - `bool`
pub struct Bool;

impl CairoType for Bool {
    type RustType = bool;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![FieldElement::from(*rust as u32)]
    }

    fn deserialize(felts: &[FieldElement]) -> Result<Self::RustType> {
        if felts[0] == FieldElement::ONE {
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

    fn deserialize(felts: &[FieldElement]) -> Result<Self::RustType> {
        // TODO: that's ugly or fine? We do know felt is always &[u8; 32]
        // byte array.
        let bytes: &[u8; 4] = &felts[0].to_bytes_be()[28..]
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

    fn deserialize(felts: &[FieldElement]) -> Result<Self::RustType> {
        // TODO: that's ugly or fine? We do know felt is always &[u8; 32]
        // byte array.
        let bytes: &[u8; 16] = &felts[0].to_bytes_be()[16..]
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

    fn deserialize(felts: &[FieldElement]) -> Result<Self::RustType> {
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
        assert_eq!(Bool::deserialize(&felts).unwrap(), false);
        assert_eq!(Bool::deserialize(&felts[1..]).unwrap(), true);
        assert_eq!(Bool::deserialize(&felts[2..]).unwrap(), false);
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
        assert_eq!(U32::deserialize(&felts).unwrap(), 123);
        assert_eq!(U32::deserialize(&felts[1..]).unwrap(), 99);
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
        assert_eq!(U128::deserialize(&felts).unwrap(), 123);
        assert_eq!(U128::deserialize(&felts[1..]).unwrap(), 99);
    }

    #[test]
    fn test_serialize_option() {
        let v = Some(32_u32);
        let felts = CairoOption::<U32>::serialize(&v);
        assert_eq!(felts.len(), 2);
        assert_eq!(felts[0], FieldElement::ZERO);
        assert_eq!(felts[1], FieldElement::from(32_u32));
    }

    #[test]
    fn test_serialize_array() {
        //let v: Vec<FieldElement> = vec![FieldElement::ZERO, FieldElement::ONE];
        let v: Vec<u32> = vec![1, 2, 3];
        let felts = Array::<U32>::serialize(&v);
        assert_eq!(felts.len(), 4);
        assert_eq!(felts[0], FieldElement::from(3_u32));
        assert_eq!(felts[1], FieldElement::ONE);
        assert_eq!(felts[2], FieldElement::TWO);
        assert_eq!(felts[3], FieldElement::THREE);

        let v: Vec<U256> = vec![
            U256 {
                low: 1,
                high: 0,
            },
            U256 {
                low: 2,
                high: 0,
            }
        ];
        let felts = Array::<U256>::serialize(&v);
        assert_eq!(felts.len(), 5);
        assert_eq!(felts[0], FieldElement::from(2_u32));
        assert_eq!(felts[1], FieldElement::ONE);
        assert_eq!(felts[2], FieldElement::ZERO);
        assert_eq!(felts[3], FieldElement::TWO);
        assert_eq!(felts[4], FieldElement::ZERO);
    }

}
