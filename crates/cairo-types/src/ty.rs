use crate::{Result};
use alloc::{vec::Vec};
use starknet::core::types::FieldElement;

// TODO: add the serialize_to method to only push data
// to an existing vec:
// serialize_to(rust: &Self::RustType, out: &mut Vec<FieldElement>).

pub trait CairoType {
    /// The corresponding Rust type.
    type RustType;

    // TODO: Check allow Fixed len + dynamic len "token".
    // As for some struct, we will know the number of felts
    // to serialize them at compile time, but when they contain
    // at least 1 array, then the length is known at runtime only.

    /// Serializes the given type into a FieldElement sequence.
    fn serialize(rust: &Self::RustType) -> Vec<FieldElement>;

    // TODO: in all deserialize implementation, add error handling
    // properly to ensure the input vec of felts is long enough
    // for deserialization.

    /// Serializes the given type into a FieldElement sequence.
    ///
    /// TODO: mutable reference may be replaced by a decoder struct
    /// or other better design pattern.
    fn deserialize(felts: &[FieldElement], offset: &mut usize) -> Result<Self::RustType>;
}
