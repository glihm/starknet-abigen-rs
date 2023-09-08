use crate::expand::utils::{str_to_ident, str_to_type};
use crate::Expandable;

use cairo_type_parser::abi_type::AbiType;
use cairo_type_parser::CairoStruct;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl Expandable for CairoStruct {
    fn expand_decl(&self) -> TokenStream2 {
        let struct_name = str_to_ident(&self.name.get_type_name_only());

        let mut members: Vec<TokenStream2> = vec![];
        for (name, member) in &self.members {
            let name = str_to_ident(&name);
            let ty = str_to_type(&member.to_rust_type());

            members.push(quote!(#name: #ty));
        }

        quote! {
            #[derive(Debug, PartialEq)]
            pub struct #struct_name {
                #(#members),*
            }
        }
    }

    fn expand_impl(&self) -> TokenStream2 {
        let struct_name = str_to_ident(&self.name.get_type_name_only());

        let mut sizes: Vec<TokenStream2> = vec![];
        let mut sers: Vec<TokenStream2> = vec![];
        let mut desers: Vec<TokenStream2> = vec![];
        let mut names: Vec<TokenStream2> = vec![];

        let mut is_first = true;
        for (name, member) in &self.members {
            let name = str_to_ident(&name);
            names.push(quote!(#name));

            let ty = str_to_type(&member.to_rust_item_path());

            // Tuples type used as rust type path must be surrounded
            // by LT/GT.
            let ty_punctuated = match member {
                AbiType::Tuple(_) => quote!(<#ty>),
                _ => quote!(#ty),
            };

            if is_first {
                sizes.push(quote!(#ty_punctuated::serialized_size(&rust.#name)));
                is_first = false;
            } else {
                sizes.push(quote!(+ #ty_punctuated::serialized_size(&rust.#name)));
            }

            sers.push(quote!(out.extend(#ty_punctuated::serialize(&rust.#name));));

            desers.push(quote! {
                let #name = #ty_punctuated::deserialize(felts, offset)?;
                offset += #ty_punctuated::serialized_size(&#name);
            });
        }

        quote! {
            impl CairoType for #struct_name {
                type RustType = Self;

                const SERIALIZED_SIZE: std::option::Option<usize> = None;

                #[inline]
                fn serialized_size(rust: &Self::RustType) -> usize {
                    #(#sizes) *
                }

                fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
                    let mut out: Vec<FieldElement> = vec![];
                    #(#sers)*
                    out
                }

                fn deserialize(felts: &[FieldElement], offset: usize) -> cairo_types::Result<Self::RustType> {
                    let mut offset = offset;
                    #(#desers)*
                    Ok(#struct_name {
                        #(#names),*
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cairo_type_parser::abi_type::AbiType;
    use cairo_type_parser::CairoStruct;

    #[test]
    fn test_decl_basic() {
        let cs = CairoStruct {
            name: AbiType::Basic("MyStruct".to_string()),
            members: vec![
                ("a".to_string(), AbiType::Basic("core::felt252".to_string())),
                (
                    "b".to_string(),
                    AbiType::Basic("core::integer::u64".to_string()),
                ),
            ],
        };

        let target = quote! {
            #[derive(Debug, PartialEq)]
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
            members: vec![(
                "a".to_string(),
                AbiType::Tuple(vec![
                    AbiType::Basic("core::felt252".to_string()),
                    AbiType::Basic("core::integer::u8".to_string()),
                ]),
            )],
        };

        let target = quote! {
            #[derive(Debug, PartialEq)]
            pub struct MyStruct {
                a: (starknet::core::types::FieldElement, u8)
            }
        };

        let ts = cs.expand_decl();
        assert_eq!(ts.to_string(), target.to_string());
    }

    #[test]
    fn test_impl_basic() {
        let cs = CairoStruct {
            name: AbiType::Basic("MyStruct".to_string()),
            members: vec![
                ("a".to_string(), AbiType::Basic("core::felt252".to_string())),
                (
                    "b".to_string(),
                    AbiType::Basic("core::integer::u64".to_string()),
                ),
            ],
        };

        #[rustfmt::skip]
        let target = quote! {
            impl CairoType for MyStruct {
                type RustType = Self ;

                const SERIALIZED_SIZE : std :: option :: Option < usize > = None ;
                #[inline] fn serialized_size (rust : & Self :: RustType) -> usize {
                    starknet::core::types::FieldElement::serialized_size(&rust.a)
                        + u64::serialized_size(&rust.b)
                }

                fn serialize (rust: &Self::RustType) -> Vec <FieldElement> {
                    let mut out : Vec < FieldElement > = vec ! [];
                    out.extend(starknet::core::types::FieldElement::serialize(&rust.a));
                    out.extend(u64::serialize(&rust.b));
                    out
                }

                fn deserialize(felts: &[FieldElement], offset: usize) -> cairo_types::Result<Self::RustType> {
                    let mut offset = offset;
                    let a = starknet::core::types::FieldElement::deserialize(felts, offset)?;
                    offset += starknet::core::types::FieldElement::serialized_size(&a);
                    let b = u64::deserialize(felts, offset)?;
                    offset += u64::serialized_size(&b);

                    Ok(MyStruct {a , b} )
                }
            }
        };

        let ts = cs.expand_impl();

        assert_eq!(ts.to_string(), target.to_string());
    }
}
