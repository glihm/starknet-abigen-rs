use crate::{CairoType, Result};
use alloc::vec::Vec;
use starknet::core::types::FieldElement;

/// ContractAddress.
#[derive(Debug, PartialEq)]
pub struct ContractAddress(pub FieldElement);

impl From<FieldElement> for ContractAddress {
    fn from(item: FieldElement) -> Self {
        Self(item)
    }
}

impl CairoType for ContractAddress {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        FieldElement::serialize(&rust.0)
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        Ok(ContractAddress(FieldElement::deserialize(felts, offset)?))
    }
}

/// ClassHash.
#[derive(Debug, PartialEq)]
pub struct ClassHash(pub FieldElement);

impl From<FieldElement> for ClassHash {
    fn from(item: FieldElement) -> Self {
        Self(item)
    }
}

impl CairoType for ClassHash {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        FieldElement::serialize(&rust.0)
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        Ok(ClassHash(FieldElement::deserialize(felts, offset)?))
    }
}

/// EthAddress.
#[derive(Debug, PartialEq)]
pub struct EthAddress(pub FieldElement);

impl From<FieldElement> for EthAddress {
    fn from(item: FieldElement) -> Self {
        Self(item)
    }
}

impl CairoType for EthAddress {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        FieldElement::serialize(&rust.0)
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        Ok(EthAddress(FieldElement::deserialize(felts, offset)?))
    }
}
