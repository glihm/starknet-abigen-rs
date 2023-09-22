use super::{AbiType, AbiTypeAny};

#[derive(Debug, PartialEq, Clone)]
pub struct AbiBasic {
    pub cairo_type: String,
    pub genty: String,
}

impl AbiBasic {
    pub fn new(cairo_type: &str) -> Self {
        AbiBasic {
            cairo_type: cairo_type.to_string(),
            genty: String::new(),
        }
    }

    fn to_rust_or_cairo_builtin_type(&self) -> String {
        let s = self.get_cairo_type_name_only();
        match s.as_str() {
            "felt252" => "starknet::core::types::FieldElement".to_string(),
            "ContractAddress" => "cairo_types::types::starknet::ContractAddress".to_string(),
            "ClassHash" => "cairo_types::types::starknet::ClassHash".to_string(),
            "EthAddress" => "cairo_types::types::starknet::EthAddress".to_string(),
            _ => s.clone(),
        }
    }
}

impl From<&str> for AbiBasic {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<&String> for AbiBasic {
    fn from(s: &String) -> Self {
        Self::new(s)
    }

}

impl AbiType for AbiBasic {
    fn get_genty(&self) -> String {
        return self.genty.clone();
    }

    fn set_genty(&mut self, genty: &str) {
        if &self.genty != "_" {
            self.genty = genty.to_string();
        }
    }

    fn compare_generic(&mut self, other: &AbiTypeAny) {
        if &self.genty != "_" {
            self.genty = other.get_genty();
        }
    }

    fn get_generic_for(&mut self, cairo_types_gentys: Vec<(&str, &str)>) -> (String, bool) {
        // A basic type can only match one of the given types.
        // It will return the first match we can find, if any.
        for (cairo_type, genty) in cairo_types_gentys {
            if self.cairo_type == cairo_type {
                self.genty = genty.to_string();
                return (genty.to_string(), true);
            }
        }

        self.genty = "_".to_string();
        (self.cairo_type.clone(), false)
    }

    fn get_cairo_type_full(&self) -> String {
        self.cairo_type.clone()
    }

    fn get_cairo_type_name_only(&self) -> String {
        self.cairo_type
            .split("::")
            .last()
            .unwrap_or(&self.cairo_type)
            .to_string()
    }

    fn to_rust_type(&self) -> String {
        if !self.genty.is_empty() && &self.genty != "_" {
            self.genty.clone()
        } else {
            self.to_rust_or_cairo_builtin_type()
        }
    }

    fn to_rust_type_path(&self) -> String {
        self.to_rust_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi_types::AbiTypeAny;

    fn get_default() -> AbiBasic {
        AbiBasic::new("core::felt252")
    }

    #[test]
    fn get_cairo_type_full() {
        let t = get_default();
        assert_eq!(t.get_cairo_type_full(), "core::felt252");
    }

    #[test]
    fn cairo_type_name_only() {
        let t = get_default();
        assert_eq!(t.get_cairo_type_name_only(), "felt252");
    }

    #[test]
    fn to_rust_type() {
        let t = get_default();
        assert_eq!(t.to_rust_type(), "starknet::core::types::FieldElement");
    }

    #[test]
    fn to_rust_type_path() {
        let t = get_default();
        assert_eq!(t.to_rust_type_path(), "starknet::core::types::FieldElement");
    }
    // TODO: add more tests for other built-in types.

    #[test]
    fn from_string() {
        let t = AbiTypeAny::from_string("core::felt252");
        assert_eq!(t, AbiTypeAny::Basic("core::felt252".into()));
    }

    #[test]
    fn from_string_generic() {
        let t = AbiTypeAny::from_string("core::felt252");
        assert_eq!(t.get_generic_for(vec![("core::felt252", "A")]), ("A".to_string(), true));
    }

    #[test]
    fn from_string_not_generic() {
        let t = AbiTypeAny::from_string("core::u32");
        assert_eq!(t.get_generic_for(vec![("core::felt252", "A")]), ("core::u32".to_string(), false));
    }
}
