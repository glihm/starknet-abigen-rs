use crate::{CairoType, Result};
use alloc::{vec, vec::Vec};
use starknet::core::types::FieldElement;

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

        Ok(Self {
            low: u128::from_be_bytes(*low),
            high: u128::from_be_bytes(*high),
        })
    }
}
