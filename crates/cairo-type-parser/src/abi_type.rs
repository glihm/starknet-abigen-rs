use std::iter::Peekable;
use std::str::Chars;

// TODO: add more validation for invalid chars in a type string.

/// Abi types are strings that represent cairo types.
/// It's important to note that, due to the serialization,
/// the cairo types are flatten into the ABI json file.
///
/// TODO: change to consider Unit as a Basic type? `Basic("()".to_string())`.
#[derive(Debug, PartialEq)]
enum AbiType {
    Basic(String),
    Nested(String, Vec<AbiType>),
    Tuple(Vec<AbiType>),
    Unit,
}

impl AbiType {
    /// Creates an [`AbiType`] from a string.
    fn from_string(type_string: &str) -> AbiType {
        let mut chars = type_string.chars().peekable();
        Self::parse_type(&mut chars)
    }

    /// Parses characters of a string to extract [`AbiType`].
    fn parse_type(chars: &mut Peekable<Chars>) -> AbiType {
        let mut nested_types = Vec::new();
        let mut current_type = String::new();
        let mut in_nested = false;
        let mut in_tuple = false;

        while let Some(c) = chars.peek() {
            match c {
                '<' => {
                    chars.next();
                    // In cairo, a nested type is always preceded by a separator "::".
                    let nested_type =
                        Self::parse_nested(&current_type.trim_end_matches("::"), chars);
                    nested_types.push(nested_type);
                    in_nested = true;
                    current_type.clear();
                }
                '>' => {
                    if in_nested {
                        chars.next();
                        in_nested = false;
                    } else {
                        break;
                    }
                }
                '(' => {
                    chars.next();
                    let tuple_type = Self::parse_tuple(chars);
                    nested_types.push(tuple_type);
                    in_tuple = true;
                }
                ')' => {
                    if in_tuple {
                        chars.next();
                        in_tuple = false;
                    } else {
                        break;
                    }
                }
                ',' => {
                    if in_tuple {
                        chars.next();
                    } else {
                        break;
                    }
                }
                ' ' => {
                    // Ignore white spaces.
                    chars.next();
                }
                _ => {
                    current_type.push(*c);
                    chars.next();
                }
            }
        }

        if !current_type.is_empty() {
            nested_types.push(AbiType::Basic(current_type.clone()));
        }

        if nested_types.is_empty() {
            // TODO: check if this one may be handled as Basic("()");
            AbiType::Unit
        } else if nested_types.len() == 1 {
            // Basic or Nested.
            nested_types.pop().unwrap()
        } else if chars.nth(0) == Some('(') {
            // Tuple.
            AbiType::Tuple(nested_types)
        } else {
            unreachable!()
        }
    }

    /// Parses a [`AbiType::Nested`] type.
    fn parse_nested(current_type: &str, chars: &mut Peekable<Chars>) -> AbiType {
        let mut nested_elements = Vec::new();

        while let Some(c) = chars.peek() {
            match c {
                '>' => {
                    chars.next();
                    break;
                }
                _ => {
                    nested_elements.push(Self::parse_type(chars));
                }
            }
        }

        AbiType::Nested(current_type.to_string(), nested_elements)
    }

    /// Parses a [`AbiType::Tuple`] type.
    fn parse_tuple(chars: &mut Peekable<Chars>) -> AbiType {
        let mut tuple_values = Vec::new();

        if let Some(_) = chars.next_if(|&x| x == ')') {
            // TODO: check if this one may be changed to `Basic("()")`.
            return AbiType::Unit;
        }

        while let Some(c) = chars.peek() {
            match c {
                ',' => {
                    chars.next();
                }
                ')' => {
                    chars.next();
                    break;
                }
                _ => {
                    tuple_values.push(Self::parse_type(chars));
                }
            }
        }

        AbiType::Tuple(tuple_values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_type() {
        let abi_type = AbiType::from_string("u32");
        assert_eq!(abi_type, AbiType::Basic("u32".to_string()));
    }

    #[test]
    fn test_basic_type_module() {
        let abi_type = AbiType::from_string("core::felt252");
        assert_eq!(abi_type, AbiType::Basic("core::felt252".to_string()));
    }

    #[test]
    fn test_unit_type() {
        let abi_type = AbiType::from_string("()");
        assert_eq!(abi_type, AbiType::Unit);
    }

    #[test]
    fn test_nested_type() {
        let abi_type = AbiType::from_string("mytype<u32>");
        let expected = AbiType::Nested(
            "mytype".to_string(),
            vec![AbiType::Basic("u32".to_string())],
        );
        assert_eq!(abi_type, expected);
    }

    #[test]
    fn test_nested_multiple_levels() {
        let abi_type = AbiType::from_string("mytype<yourtype<u32>>");
        let expected = AbiType::Nested(
            "mytype".to_string(),
            vec![AbiType::Nested(
                "yourtype".to_string(),
                vec![AbiType::Basic("u32".to_string())],
            )],
        );
        assert_eq!(abi_type, expected);
    }

    #[test]
    fn test_tuple_type() {
        let abi_type = AbiType::from_string("(u32, u64)");
        let expected = AbiType::Tuple(vec![
            AbiType::Basic("u32".to_string()),
            AbiType::Basic("u64".to_string()),
        ]);
        assert_eq!(abi_type, expected);
    }

    #[test]
    fn test_nested_tuple_type() {
        let abi_type = AbiType::from_string("mytuple<(u32, u64)>");
        let expected = AbiType::Nested(
            "mytuple".to_string(),
            vec![AbiType::Tuple(vec![
                AbiType::Basic("u32".to_string()),
                AbiType::Basic("u64".to_string()),
            ])],
        );
        assert_eq!(abi_type, expected);
    }

    #[test]
    fn test_nested_tuple_nested_type() {
        let abi_type = AbiType::from_string("core::array::Array::<(core::felt252, core::array::Array::<(core::felt252, core::felt252)>)>");
        let expected = AbiType::Nested(
            "core::array::Array".to_string(),
            vec![AbiType::Tuple(vec![
                AbiType::Basic("core::felt252".to_string()),
                AbiType::Nested(
                    "core::array::Array".to_string(),
                    vec![AbiType::Tuple(vec![
                        AbiType::Basic("core::felt252".to_string()),
                        AbiType::Basic("core::felt252".to_string()),
                    ])],
                ),
            ])],
        );
        assert_eq!(abi_type, expected);
    }

    #[test]
    fn test_tuple_nested_tuple_type() {
        let abi_type = AbiType::from_string(
            "(core::array::Span::<(core::felt252, contracts::c1::PG)>, core::felt252)",
        );
        let expected = AbiType::Tuple(vec![
            AbiType::Nested(
                "core::array::Span".to_string(),
                vec![AbiType::Tuple(vec![
                    AbiType::Basic("core::felt252".to_string()),
                    AbiType::Basic("contracts::c1::PG".to_string()),
                ])],
            ),
            AbiType::Basic("core::felt252".to_string()),
        ]);
        assert_eq!(abi_type, expected);
    }
}
