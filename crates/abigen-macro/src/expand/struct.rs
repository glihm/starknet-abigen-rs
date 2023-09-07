use crate::Expandable;
use crate::expand::utils::{str_to_ident, str_to_type};

use cairo_type_parser::abi_type::AbiType;
use cairo_type_parser::{CairoAbiEntry, CairoStruct};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_quote, Ident, Type};

impl Expandable for CairoStruct {
    fn expand_decl(&self) -> TokenStream2 {
        let struct_name = str_to_ident(&self.name.get_type_name_only());

        let mut members: Vec<TokenStream2> = vec![];
        for (name, member) in &self.members {
            let name = str_to_ident(&name);
            let ty = str_to_type(&member.to_rust_type());

            members.push(quote!(#name: #ty));
        }

        quote!{
            #[derive(Debug)]
            pub struct #struct_name {
                #(#members),*
            }
        }
    }

    fn expand_impl(&self) -> TokenStream2 {
        quote!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cairo_type_parser::abi_type::AbiType;
    use cairo_type_parser::{CairoAbiEntry, CairoStruct};

    #[test]
    fn test_decl_basic() {
        let cs = CairoStruct {
            name: AbiType::Basic("MyStruct".to_string()),
            members: vec![
                (
                    "a".to_string(),
                    AbiType::Basic("core::felt252".to_string())
                ),
                (
                    "b".to_string(),
                    AbiType::Basic("core::integer::u64".to_string())
                ),
            ]
        };

        let target = quote! {
            #[derive(Debug)]
            pub struct MyStruct {
                a: starknet::core::types::FieldElement,
                b: u64
            }
        };

        let ts = cs.expand_decl();

        assert_eq!(ts.to_string(), target.to_string());
    }

    #[test]
    fn test_decl_tuple() {
        let cs = CairoStruct {
            name: AbiType::Basic("MyStruct".to_string()),
            members: vec![
                (
                    "a".to_string(),
                    AbiType::Tuple(vec![
                        AbiType::Basic("core::felt252".to_string()),
                        AbiType::Basic("core::integer::u8".to_string()),
                    ])
                )
            ]
        };

        let target = quote! {
            #[derive(Debug)]
            pub struct MyStruct {
                a: (starknet::core::types::FieldElement, u8)
            }
        };

        let ts = cs.expand_decl();
        assert_eq!(ts.to_string(), target.to_string());
    }
}
