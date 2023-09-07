pub mod abi_type;

use proc_macro2::TokenStream as TokenStream2;
use std::collections::HashMap;
use syn::Type;

pub enum TokenType {
    Unit,
    Felt,
    Array(Box<ArrayToken>),
    Tuple(TupleToken),
    Struct(StructToken),
    Enum(EnumToken),
    Function(FunctionToken),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
    }

    #[test]
    fn test_tokenize_simple() {}
}
