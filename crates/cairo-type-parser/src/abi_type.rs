use std::iter::Peekable;
use std::str::Chars;

// TODO: add more validation for invalid chars in a type string.

// TODO: change String into &str.

/// Abi types are strings that represent cairo types.
/// It's important to note that, due to the serialization,
/// the cairo types are flatten into the ABI json file.
#[derive(Debug, PartialEq, Clone)]
pub enum AbiType {
    Basic(String),
    // TODO: is there a better way to avoid infinite recursion without box?
    // Array and span are handled by the same struct.
    Array(Box<AbiType>),
    Nested(String, Vec<AbiType>),
    Tuple(Vec<AbiType>),
}

impl AbiType {
    /// Extracts the type name from a string,
    /// ignoring the modules.
    ///
    /// # Examples
    ///
    /// core::felt252 -> felt252
    /// core::array::Array -> Array
    pub fn get_type_name_only(&self) -> String {
        let name = match self {
            AbiType::Basic(n) => n,
            AbiType::Nested(n, _) => n,
            AbiType::Array(_) => return "|array|".to_string(),
            AbiType::Tuple(_) => return "|tuple|".to_string(),
        };

        name.split("::").last().unwrap_or(name).to_string()
    }

    pub fn to_rust_type(&self) -> String {
        self.to_rust_type_punc("<")
    }

    pub fn to_rust_type_path(&self) -> String {
        self.to_rust_type_punc("::<")
    }

    /// TODO: check if this can be factorize in some way with the function above.
    /// Like passing if we want the type_path or not, and the is_first.
    fn to_rust_type_punc(&self, open_punc: &str) -> String {
        let mut rust_type = String::new();
        let type_str = self.get_type_name_only();

        if let AbiType::Basic(_) = self {
            return AbiType::to_rust_basic_types(&type_str);
        };

        // Only Tuple, Array or Nested from here.
        match type_str.as_str() {
            "|tuple|" => rust_type.push('('),
            "|array|" => rust_type.push_str(format!("Vec{open_punc}").as_str()),
            _ => {
                // Structs can be nested, but the type is flatten
                // for each member. So we can only return the type name.
                return type_str.to_string();
            }
        };

        match self {
            AbiType::Nested(_, inners) => {
                for inner in inners {
                    if open_punc == "::<" {
                        rust_type.push_str(&inner.to_rust_type_path())
                    } else {
                        rust_type.push_str(&inner.to_rust_type())
                    }
                }
            }
            AbiType::Array(inner) => {
                if open_punc == "::<" {
                    rust_type.push_str(&inner.to_rust_type_path())
                } else {
                    rust_type.push_str(&inner.to_rust_type())
                }
            }
            AbiType::Tuple(inners) => {
                for (idx, inner) in inners.iter().enumerate() {
                    if open_punc == "::<" {
                        rust_type.push_str(&inner.to_rust_type_path());
                    } else {
                        rust_type.push_str(&inner.to_rust_type());
                    }

                    if idx < inners.len() - 1 {
                        rust_type.push_str(", ");
                    }
                }
            }
            _ => (),
        };

        match type_str.as_str() {
            "|tuple|" => rust_type.push(')'),
            "|array|" => rust_type.push('>'),
            _ => (), // Nothing to do here, we only close nested tuple/array.
        }

        rust_type.to_string()
    }

    /// Creates an [`AbiType`] from a string.
    pub fn from_string(type_string: &str) -> AbiType {
        let mut chars = type_string.chars().peekable();
        Self::parse_type(&mut chars)
    }

    ///
    fn to_rust_basic_types(type_string: &str) -> String {
        match type_string {
            "felt252" => "starknet::core::types::FieldElement".to_string(),
            "ContractAddress" => "cairo_types::types::starknet::ContractAddress".to_string(),
            "ClassHash" => "cairo_types::types::starknet::ClassHash".to_string(),
            "EthAddress" => "cairo_types::types::starknet::EthAddress".to_string(),
            _ => type_string.to_string(),
        }
    }

    /// Parses characters of a string to extract [`AbiType`].
    fn parse_type(chars: &mut Peekable<Chars>) -> AbiType {
        let mut nested_types = Vec::new();
        let mut current_type = String::new();
        let mut in_nested = false;

        while let Some(c) = chars.peek() {
            match c {
                '<' => {
                    chars.next();
                    // In cairo, a nested type is always preceded by a separator "::".
                    let nested_type =
                        Self::parse_nested(current_type.trim_end_matches("::"), chars);
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
                }
                ')' => {
                    break;
                }
                ',' => {
                    break;
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
            AbiType::Basic("()".to_string())
        } else if nested_types.len() == 1 {
            // Basic, Array or Nested with 1 inner type.
            nested_types.pop().unwrap()
        } else if chars.next() == Some('(') {
            // Tuple.
            AbiType::Tuple(nested_types)
        } else {
            // Nested types into a nested type.
            println!("NESTED_TYPES? {:?}", nested_types);
            unreachable!();
            //AbiType::Nested(current_type.to_string(), nested_types)
        }
    }

    /// Parses a [`AbiType::Nested`] type.
    fn parse_nested(current_type: &str, chars: &mut Peekable<Chars>) -> AbiType {
        let mut inners = vec![];

        let is_array = current_type == "core::array::Array" || current_type == "core::array::Span";

        while let Some(c) = chars.peek() {
            match c {
                '>' => {
                    chars.next();
                    break;
                }
                ',' => {
                    chars.next();
                    inners.push(Self::parse_type(chars))
                }
                _ => {
                    inners.push(Self::parse_type(chars));
                }
            }
        }

        if inners.is_empty() {
            panic!("Array/Nested type expects at least one inner type");
        }

        if is_array {
            if inners.len() == 1 {
                AbiType::Array(Box::new(inners[0].clone()))
            } else {
                panic!("Array/Span expect exactly one inner type");
            }
        } else {
            AbiType::Nested(current_type.to_string(), inners)
        }
    }

    /// Parses a [`AbiType::Tuple`] type.
    fn parse_tuple(chars: &mut Peekable<Chars>) -> AbiType {
        let mut tuple_values = Vec::new();

        if chars.next_if(|&x| x == ')').is_some() {
            // TODO: check if this one may be changed to `Basic("()")`.
            return AbiType::Basic("()".to_string());
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
    fn test_get_type_name_only() {
        let t = AbiType::Basic("u32".to_string());
        assert_eq!(t.get_type_name_only(), "u32");

        let t = AbiType::Nested(
            "MyStruct".to_string(),
            vec![AbiType::Basic("u32".to_string())],
        );
        assert_eq!(t.get_type_name_only(), "MyStruct");

        let t = AbiType::Tuple(vec![]);
        assert_eq!(t.get_type_name_only(), "|tuple|");

        let t = AbiType::Basic("()".to_string());
        assert_eq!(t.get_type_name_only(), "()");

        let t = AbiType::Array(Box::new(AbiType::Basic("core::felt252".to_string())));
        assert_eq!(t.get_type_name_only(), "|array|");
    }

    #[test]
    fn test_get_type_name_only_modules() {
        let t = AbiType::Basic("core::felt252".to_string());
        assert_eq!(t.get_type_name_only(), "felt252");

        let t = AbiType::Array(Box::new(AbiType::Basic("core::felt252".to_string())));
        assert_eq!(t.get_type_name_only(), "|array|");
    }

    #[test]
    fn test_to_rust_type_basic() {
        let t = AbiType::Basic("()".to_string());
        assert_eq!(t.to_rust_type(), "()");

        let t = AbiType::Basic("core::felt252".to_string());
        assert_eq!(t.to_rust_type(), "starknet::core::types::FieldElement");

        let t = AbiType::Basic("u32".to_string());
        assert_eq!(t.to_rust_type(), "u32");
    }

    #[test]
    fn test_to_rust_type_tuple() {
        let t = AbiType::Tuple(vec![
            AbiType::Basic("core::felt252".to_string()),
            AbiType::Basic("core::integer::u32".to_string()),
        ]);
        assert_eq!(
            t.to_rust_type(),
            "(starknet::core::types::FieldElement, u32)"
        );

        let t = AbiType::Tuple(vec![
            AbiType::Basic("core::felt252".to_string()),
            AbiType::Basic("core::integer::u32".to_string()),
            AbiType::Basic("core::integer::u8".to_string()),
        ]);
        assert_eq!(
            t.to_rust_type(),
            "(starknet::core::types::FieldElement, u32, u8)"
        );
    }

    #[test]
    fn test_to_rust_type_nested() {
        let t = AbiType::Array(Box::new(AbiType::Basic("core::felt252".to_string())));
        assert_eq!(t.to_rust_type(), "Vec<starknet::core::types::FieldElement>");

        let t = AbiType::Array(Box::new(AbiType::Array(Box::new(AbiType::Basic(
            "core::felt252".to_string(),
        )))));
        assert_eq!(
            t.to_rust_type(),
            "Vec<Vec<starknet::core::types::FieldElement>>"
        );

        let t = AbiType::Nested(
            "mod1::mod2::MyStruct".to_string(),
            vec![AbiType::Basic("u32".to_string())],
        );
        assert_eq!(t.to_rust_type(), "MyStruct");
    }

    #[test]
    fn test_to_rust_type_nested_tuple() {
        let t = AbiType::Array(Box::new(AbiType::Tuple(vec![
            AbiType::Basic("core::felt252".to_string()),
            AbiType::Basic("core::integer::u128".to_string()),
        ])));
        assert_eq!(
            t.to_rust_type(),
            "Vec<(starknet::core::types::FieldElement, u128)>"
        );
    }

    #[test]
    fn test_to_rust_type_path_nested() {
        let t = AbiType::Array(Box::new(AbiType::Basic("core::felt252".to_string())));
        assert_eq!(
            t.to_rust_type_path(),
            "Vec::<starknet::core::types::FieldElement>"
        );

        let t = AbiType::Array(Box::new(AbiType::Array(Box::new(AbiType::Basic(
            "core::felt252".to_string(),
        )))));
        assert_eq!(
            t.to_rust_type_path(),
            "Vec::<Vec::<starknet::core::types::FieldElement>>"
        );
    }

    #[test]
    fn test_to_rust_type_path_tuple() {
        let t = AbiType::Tuple(vec![
            AbiType::Basic("core::felt252".to_string()),
            AbiType::Basic("core::integer::u128".to_string()),
        ]);
        assert_eq!(
            t.to_rust_type_path(),
            "(starknet::core::types::FieldElement, u128)"
        );

        let t = AbiType::Array(Box::new(AbiType::Tuple(vec![
            AbiType::Basic("core::felt252".to_string()),
            AbiType::Basic("core::integer::u32".to_string()),
        ])));
        assert_eq!(
            t.to_rust_type_path(),
            "Vec::<(starknet::core::types::FieldElement, u32)>"
        );

        let t = AbiType::Tuple(vec![
            AbiType::Array(Box::new(AbiType::Basic("core::felt252".to_string()))),
            AbiType::Basic("core::integer::u32".to_string()),
        ]);
        assert_eq!(
            t.to_rust_type_path(),
            "(Vec::<starknet::core::types::FieldElement>, u32)"
        );
    }

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
        assert_eq!(abi_type, AbiType::Basic("()".to_string()));
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
    fn test_nested_various() {
        let abi_type = AbiType::from_string("mytype<t1, t2>");
        let expected = AbiType::Nested(
            "mytype".to_string(),
            vec![
                AbiType::Basic("t1".to_string()),
                AbiType::Basic("t2".to_string()),
            ],
        );
        assert_eq!(abi_type, expected);
    }

    #[test]
    fn test_nested_various_module() {
        let abi_type = AbiType::from_string("core::result::Result<(), core::felt252>");
        let expected = AbiType::Nested(
            "core::result::Result".to_string(),
            vec![
                AbiType::Basic("()".to_string()),
                AbiType::Basic("core::felt252".to_string()),
            ],
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
        let expected = AbiType::Array(Box::new(AbiType::Tuple(vec![
            AbiType::Basic("core::felt252".to_string()),
            AbiType::Array(Box::new(AbiType::Tuple(vec![
                AbiType::Basic("core::felt252".to_string()),
                AbiType::Basic("core::felt252".to_string()),
            ]))),
        ])));
        assert_eq!(abi_type, expected);
    }

    #[test]
    fn test_tuple_nested_tuple_type() {
        let abi_type = AbiType::from_string(
            "(core::array::Span::<(core::felt252, contracts::c1::PG)>, core::felt252)",
        );
        let expected = AbiType::Tuple(vec![
            AbiType::Array(Box::new(AbiType::Tuple(vec![
                AbiType::Basic("core::felt252".to_string()),
                AbiType::Basic("contracts::c1::PG".to_string()),
            ]))),
            AbiType::Basic("core::felt252".to_string()),
        ]);
        assert_eq!(abi_type, expected);
    }
}
