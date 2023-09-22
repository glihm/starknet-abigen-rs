use std::iter::Peekable;
use std::str::Chars;

pub mod basic;
pub use basic::AbiBasic;

pub mod array;
pub use array::AbiArray;

pub mod generic;
pub use generic::AbiGeneric;

pub mod tuple;
pub use tuple::AbiTuple;

// TODO: if we want to process Option or Result as built-in,
// we need here a new variant for each. With this in mind, we will
// be able to handle them correctly.
// For now, they are generated on the fly an considered as Basic or Generic.

#[derive(Debug, PartialEq, Clone)]
pub enum AbiTypeAny {
    Basic(AbiBasic),
    Array(AbiArray),
    // Generics for struct and enums.
    Generic(AbiGeneric),
    Tuple(AbiTuple),
}

pub trait AbiType {
    fn get_genty(&self) -> String;
    fn set_genty(&mut self, genty: &str);

    fn compare_generic(&mut self, other: &AbiTypeAny);

    fn get_generic_for(&mut self, cairo_types_gentys: Vec<(&str, &str)>) -> (String, bool);

    fn get_cairo_type_full(&self) -> String;

    fn get_cairo_type_name_only(&self) -> String;

    fn to_rust_type(&self) -> String;

    fn to_rust_type_path(&self) -> String;
}

impl AbiType for AbiTypeAny {
    // TODO: do we want to accept any kind, and it's the type itself that
    // made the comparison?
    fn compare_generic(&mut self, other: &AbiTypeAny) {
        match self {
            AbiTypeAny::Basic(a) => a.compare_generic(other),
            AbiTypeAny::Array(a) => a.compare_generic(other),
            AbiTypeAny::Generic(a) => a.compare_generic(other),
            AbiTypeAny::Tuple(a) => a.compare_generic(other),
        }
    }

    fn set_genty(&mut self, genty: &str) {
        match self {
            AbiTypeAny::Basic(a) => a.set_genty(genty),
            AbiTypeAny::Array(a) => a.set_genty(genty),
            AbiTypeAny::Generic(a) => a.set_genty(genty),
            AbiTypeAny::Tuple(a) => a.set_genty(genty),
        }
    }

    fn get_genty(&self) -> String {
        match self {
            AbiTypeAny::Basic(a) => a.get_genty(),
            AbiTypeAny::Array(a) => a.get_genty(),
            AbiTypeAny::Generic(a) => a.get_genty(),
            AbiTypeAny::Tuple(a) => a.get_genty(),
        }
    }

    fn get_generic_for(&mut self, cairo_types_gentys: Vec<(&str, &str)>) -> (String, bool) {
        match self {
            AbiTypeAny::Basic(a) => a.get_generic_for(cairo_types_gentys),
            AbiTypeAny::Array(a) => a.get_generic_for(cairo_types_gentys),
            AbiTypeAny::Generic(a) => a.get_generic_for(cairo_types_gentys),
            AbiTypeAny::Tuple(a) => a.get_generic_for(cairo_types_gentys),
        }
    }

    fn get_cairo_type_full(&self) -> String {
        match self {
            AbiTypeAny::Basic(a) => a.get_cairo_type_full(),
            AbiTypeAny::Array(a) => a.get_cairo_type_full(),
            AbiTypeAny::Generic(a) => a.get_cairo_type_full(),
            AbiTypeAny::Tuple(a) => a.get_cairo_type_full(),
        }
    }

    fn get_cairo_type_name_only(&self) -> String {
        match self {
            AbiTypeAny::Basic(a) => a.get_cairo_type_name_only(),
            AbiTypeAny::Array(a) => a.get_cairo_type_name_only(),
            AbiTypeAny::Generic(a) => a.get_cairo_type_name_only(),
            AbiTypeAny::Tuple(a) => a.get_cairo_type_name_only(),
        }
    }

    fn to_rust_type(&self) -> String {
        match self {
            AbiTypeAny::Basic(a) => a.to_rust_type(),
            AbiTypeAny::Array(a) => a.to_rust_type(),
            AbiTypeAny::Generic(a) => a.to_rust_type(),
            AbiTypeAny::Tuple(a) => a.to_rust_type(),
        }
    }

    fn to_rust_type_path(&self) -> String {
        match self {
            AbiTypeAny::Basic(a) => a.to_rust_type_path(),
            AbiTypeAny::Array(a) => a.to_rust_type_path(),
            AbiTypeAny::Generic(a) => a.to_rust_type_path(),
            AbiTypeAny::Tuple(a) => a.to_rust_type_path(),
        }
    }
}

impl AbiTypeAny {

    pub fn is_generic(&self) -> bool {
        match self {
            AbiTypeAny::Generic(_) => true,
            _ => false
        }
    }

    pub fn is_unit(&self) -> bool {
        match self {
            AbiTypeAny::Basic(b) => {
                if b.get_cairo_type_full() == "()" {
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }

    pub fn from_string(type_string: &str) -> AbiTypeAny {
        let mut chars = type_string.chars().peekable();
        Self::parse_type(&mut chars)
    }

    fn parse_type(chars: &mut Peekable<Chars>) -> AbiTypeAny {
        let mut generic_types = Vec::new();
        let mut current_type = String::new();
        let mut in_generic = false;

        while let Some(c) = chars.peek() {
            match c {
                '<' => {
                    chars.next();
                    // In cairo, a generic type is always preceded by a separator "::".
                    let generic_type =
                        Self::parse_generic(&current_type.trim_end_matches("::"), chars);
                    generic_types.push(generic_type);
                    in_generic = true;
                    current_type.clear();
                }
                '>' => {
                    if in_generic {
                        chars.next();
                        in_generic = false;
                    } else {
                        break;
                    }
                }
                '(' => {
                    chars.next();
                    let tuple_type = Self::parse_tuple(chars);
                    generic_types.push(tuple_type);
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
            generic_types.push(AbiTypeAny::Basic((&current_type).into()));
        }

        if generic_types.is_empty() {
            // TODO: check if this one may be handled as Basic("()");
            AbiTypeAny::Basic("()".into())
        } else if generic_types.len() == 1 {
            // Basic, Array or Generic with 1 inner type.
            generic_types.pop().unwrap()
        } else if chars.nth(0) == Some('(') {
            // Tuple.
            AbiTypeAny::Tuple(AbiTuple::new(generic_types))
        } else {
            // Generic types into a generic type.
            println!("GENERIC_TYPES? {:?}", generic_types);
            unreachable!();
        }
    }

    fn parse_generic(current_type: &str, chars: &mut Peekable<Chars>) -> AbiTypeAny {
        let mut inners = vec![];

        let is_array = current_type.contains("core::array::Array")
            || current_type.contains("core::array::Span");

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

        if inners.len() == 0 {
            panic!("Array/Generic type expects at least one inner type");
        }

        if is_array {
            if inners.len() == 1 {
                AbiTypeAny::Array(AbiArray::new(current_type, inners[0].clone()))
            } else {
                panic!("Array/Span expect exactly one inner type");
            }
        } else {
            AbiTypeAny::Generic(AbiGeneric::new(current_type, inners))
        }
    }

    fn parse_tuple(chars: &mut Peekable<Chars>) -> AbiTypeAny {
        let mut tuple_values = Vec::new();

        if let Some(_) = chars.next_if(|&x| x == ')') {
            // TODO: check if this one may be changed to `Basic("()")`.
            return AbiTypeAny::Basic("()".into());
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

        AbiTypeAny::Tuple(AbiTuple::new(tuple_values))
    }
}
