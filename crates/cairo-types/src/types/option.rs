//! CairoType implementation for Option.
//!
//! In cairo, `Some` is the first field and `None` the second one.
//! To follow the serialization rule, `Some` has index 0, and `None` index 1.
//!
//! https://github.com/starkware-libs/cairo/blob/main/corelib/src/option.cairo#L6
use crate::error::{Error, Result};
use crate::ty::CairoType;
use alloc::string::ToString;
use alloc::{vec, vec::Vec};
use starknet::core::types::FieldElement;

impl<T, RT> CairoType for Option<T>
where
    T: CairoType<RustType = RT>,
{
    type RustType = Option<RT>;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out = vec![];

        match rust {
            Some(r) => {
                out.push(FieldElement::ZERO);
                out.extend(T::serialize(r));
            }
            None => out.push(FieldElement::ONE),
        };

        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let idx = felts[offset];

        if idx == FieldElement::ZERO {
            // + 1 as the offset value is the index of the enum.
            Ok(Option::Some(T::deserialize(felts, offset + 1)?))
        } else if idx == FieldElement::ONE {
            Ok(Option::None)
        } else {
            Err(Error::Deserialize(
                "Option is expected 0 or 1 index only".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use starknet::core::types::FieldElement;

    #[test]
    fn option_some_serialize() {
        let o = Some(u32::MAX);
        let felts = Option::<u32>::serialize(&o);
        assert_eq!(felts.len(), 2);
        assert_eq!(felts[0], FieldElement::ZERO);
        assert_eq!(felts[1], FieldElement::from(u32::MAX));
    }

    #[test]
    fn option_some_deserialize() {
        let felts = vec![FieldElement::ZERO, FieldElement::from(u32::MAX)];
        let o = Option::<u32>::deserialize(&felts, 0).unwrap();
        assert_eq!(o, Some(u32::MAX));

        let felts = vec![
            FieldElement::THREE,
            FieldElement::ZERO,
            FieldElement::from(u32::MAX),
        ];
        let o = Option::<u32>::deserialize(&felts, 1).unwrap();
        assert_eq!(o, Some(u32::MAX));
    }

    #[test]
    fn option_none_serialize() {
        let o: Option<u32> = None;
        let felts = Option::<u32>::serialize(&o);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::ONE);
    }

    #[test]
    fn option_none_deserialize() {
        let felts = vec![FieldElement::ONE];
        let o = Option::<u32>::deserialize(&felts, 0).unwrap();
        assert_eq!(o, None);

        let felts = vec![FieldElement::THREE, FieldElement::ONE];
        let o = Option::<u32>::deserialize(&felts, 1).unwrap();
        assert_eq!(o, None);
    }
}
