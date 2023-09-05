




// // AUTO GENERATED code example!
// pub struct U256 {
//     low: u128,
//     high: u128,
// }

// impl CairoType for U256 {
//     type RustType = Self;

//     fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
//         vec![
//             FieldElement::from(rust.low),
//             FieldElement::from(rust.high)
//         ]
//     }

//     fn deserialize(felts: &[FieldElement]) -> Result<Self::RustType> {
//         Ok(U256 {
//             low: 0,
//             high: 0,
//         })
//     }
// }


// /// RustOption - Example on how implementing a type that is
// /// depending on an other type using T.
// pub struct CairoOption<T: CairoType>(PhantomData<T>);

// impl<T, U> CairoType for CairoOption<T> where T: CairoType<RustType = U> {
//     type RustType = Option<U>;

//     fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
//         match rust {
//             Some(v) => {
//                 let mut felts = vec![FieldElement::ZERO];
//                 felts.extend(T::serialize(v));
//                 felts
//             }
//             None => vec![FieldElement::ONE]
//         }
//     }

//     fn deserialize(_felts: &[FieldElement]) -> Result<Self::RustType> {
//         Ok(Option::None)
//     }
// }
// // ********* EXAMPLE ****
