use crate::{Result};
use alloc::{vec::Vec};
use starknet::core::types::FieldElement;


pub trait CairoType {
    /// The corresponding Rust type.
    type RustType;

    /// Serializes the given type into a FieldElement sequence.
    fn serialize(rust: &Self::RustType) -> Vec<FieldElement>;

    /// Serializes the given type into a FieldElement sequence.
    fn deserialize(felts: &[FieldElement]) -> Result<Self::RustType>;
}
