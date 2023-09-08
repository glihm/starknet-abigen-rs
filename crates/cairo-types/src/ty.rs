use crate::Result;
use alloc::vec::Vec;
use starknet::core::types::FieldElement;

/// Basic cairo structs that are already implemented inside
/// this crate.
/// They can be ignored when abigen macro is used.
pub const CAIRO_BASIC_STRUCTS: [&str; 1] = ["Span"];
pub const CAIRO_BASIC_ENUMS: [&str; 1] = ["Option"];

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

    /// The serialized size of the type in felts, if known at compile time.
    const SERIALIZED_SIZE: Option<usize> = Some(1);

    /// Whether the serialized size is dynamic.
    const DYNAMIC: bool = Self::SERIALIZED_SIZE.is_none();

    /// Calculate the serialized size of the data For a single felt
    /// it will always be 1.
    /// If the type is dynamic, SERIALIZED_SIZE is None, but the
    /// function is overriden to correctly compute the size.
    #[inline]
    fn serialized_size(_rust: &Self::RustType) -> usize {
        Self::SERIALIZED_SIZE.unwrap()
    }

    /// Serializes the given type into a FieldElement sequence.
    fn serialize(rust: &Self::RustType) -> Vec<FieldElement>;

    // TODO: in all deserialize implementation, add error handling
    // properly to ensure the input vec of felts is long enough
    // for deserialization.

    /// Serializes the given type into a FieldElement sequence.
    ///
    /// TODO: mutable reference may be replaced by a decoder struct
    /// or other better design pattern.
    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType>;
}
