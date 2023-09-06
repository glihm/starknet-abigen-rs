use proc_macro2::TokenStream as TokenStream2;
use syn::Type;
use std::collections::HashMap;

pub enum TokenType {
    Unit,
    Felt,
    Array(Box<ArrayToken>),
    Tuple(TupleToken),
    Struct(StructToken),
    Enum(EnumToken),
    Function(FunctionToken),
    UnresolvedCustomType(UnresolvedToken),
}

/// A token that is not resolvable at
/// the moment of the parsing, but may be resolved
/// later with the ABI context.
///
/// # Examples
///
/// If a member of a struct has this type:
/// `core::array::Array::<(core::felt252, mod1::mod2::MyStruct)>`
/// the tokenize parser can't resolve the token without having
/// a lookup into the already parsed tokens.
pub struct UnresolvedToken {
    name: String,
}

pub struct ArrayToken {
    inner: TokenType,
}

pub struct TupleToken {
    values: Vec<TokenType>,
}

pub struct StructToken {
    name: String,
    members: Vec<(String, TokenType)>,
}

pub struct EnumToken {
    name: String,
    variants: Vec<(String, TokenType)>,
}

pub struct FunctionToken {
    name: String,
    inputs: Vec<(String, TokenType)>,
    outputs: Vec<TokenType>,
}

/// Gathers type details from a cairo abi type string representation and
/// returns a `HashMap` with each type found and it's depth.
///
/// # Examples
///
/// > core::felt252 results in { 0: "core::felt252" }
/// > mymod::MyStruct::<core::felt252> results in { 0: "mymod::MyStruct", 1: "core::felt252" }
fn detail_cairo_abi_type(cairo_abi_type: &str) -> HashMap<u32, String> {
    let mut types: HashMap<u32, String> = HashMap::new();
    let mut current_type = String::from("");
    let mut depth = 0;
    let mut has_depth = false;
    let mut in_tuple = false;

    for c in cairo_abi_type.chars() {
        match c {
            '(' => {
                in_tuple = true;
                current_type.push(c);
            },
            ')' => {
                in_tuple = false;
                current_type.push(c);
                types.insert(depth, String::from(current_type.clone()));
                current_type = String::from("");
            },
            '<' => {
                types.insert(depth, String::from(current_type.trim_end_matches("::")));
                depth += 1;
                has_depth = true;
                current_type = String::from("");

                // If not an array, we can skip the templated part
                // as everything is flatten in the struct/enum ABI entry?
            },
            '>' => {
                // Ensures the most inner type is included.
                if !types.contains_key(&depth) {
                    types.insert(depth, current_type.clone());
                }
                depth -= 1;
            },
            _ => {
                current_type.push(c);
            }
        }
    }

    // Includes a type without any inner type.
    if !has_depth {
        types.insert(0, current_type);
    }

    types
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let tys = detail_cairo_abi_type("ty");
        assert_eq!(tys.len(), 1);
        assert_eq!(tys[&0], "ty");

        let tys = detail_cairo_abi_type("mod::ty1");
        assert_eq!(tys.len(), 1);
        assert_eq!(tys[&0], "mod::ty1");

        let tys = detail_cairo_abi_type(
            "mod::mod2::ty1::<mod::ty2::<mod::ty3>>");
        assert_eq!(tys.len(), 3);
        assert_eq!(tys[&0], "mod::mod2::ty1");
        assert_eq!(tys[&1], "mod::ty2");
        assert_eq!(tys[&2], "mod::ty3");

        let tys = detail_cairo_abi_type(
            "core::array::Span::<(core::felt252, mod1::mod2::PG)>");
        assert_eq!(tys.len(), 2);
        assert_eq!(tys[&0], "core::array::Span");
        assert_eq!(tys[&1], "(core::felt252, mod1::mod2::PG)");
    }

    #[test]
    fn test_tokenize_simple() {
        
    }
}

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
