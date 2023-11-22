//! Dedicated struct for cairo 0 arrays, where len is not prefixed.
use crate::cairo_types::{CairoType, Result};
use starknet::core::types::FieldElement;

#[derive(Debug, Clone, PartialEq)]
pub struct CairoArrayLegacy<T>(pub Vec<T>);

impl<T: std::clone::Clone> CairoArrayLegacy<T> {
    pub fn from_slice(slice: &[T]) -> Self {
        Self(slice.to_vec())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> From<Vec<T>> for CairoArrayLegacy<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T, RT> CairoType for CairoArrayLegacy<T>
where
    T: CairoType<RustType = RT>,
{
    type RustType = CairoArrayLegacy<RT>;

    const SERIALIZED_SIZE: Option<usize> = None;

    #[inline]
    fn serialized_size(rust: &Self::RustType) -> usize {
        let data = &rust.0;
        // In cairo 0, the length is always passed as an argument.
        data.iter().map(T::serialized_size).sum::<usize>()
    }

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out: Vec<FieldElement> = vec![];
        rust.0.iter().for_each(|r| out.extend(T::serialize(r)));
        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let mut out: Vec<RT> = vec![];
        let mut offset = offset;

        loop {
            if out.len() == felts.len() {
                break;
            }

            let rust: RT = T::deserialize(felts, offset)?;
            offset += T::serialized_size(&rust);
            out.push(rust);
        }

        Ok(CairoArrayLegacy(out))
    }
}
