use crate::expand::utils::{str_to_ident, str_to_type};
use crate::Expandable;
use cairo_type_parser::CairoEnum;

use cairo_type_parser::abi_type::AbiType;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl Expandable for CairoEnum {
    fn expand_decl(&self) -> TokenStream2 {
        let enum_name = str_to_ident(&self.name.get_type_name_only());

        let mut variants: Vec<TokenStream2> = vec![];

        for (name, abi_type) in &self.variants {
            let name = str_to_ident(&name);
            let ty = str_to_type(&abi_type.to_rust_type());
            if abi_type == &AbiType::Basic("()".to_string()) {
                variants.push(quote!(#name));
            } else {
                variants.push(quote!(#name(#ty)));
            }
        }

        quote! {
            #[derive(Debug, PartialEq)]
            pub enum #enum_name {
                #(#variants),*
            }
        }
    }
    fn expand_impl(&self) -> TokenStream2 {
        let name_str = &self.name.get_type_name_only();
        let enum_name = str_to_ident(name_str);

        let mut serialized_sizes: Vec<TokenStream2> = vec![];
        let mut serializations: Vec<TokenStream2> = vec![];
        let mut deserializations: Vec<TokenStream2> = vec![];

        for (i, val) in self.variants.iter().enumerate() {
            let variant_name = str_to_ident(&val.0);
            let ty = str_to_type(&val.1.to_rust_item_path(true));
            if val.1 == AbiType::Basic("()".to_string()) {
                serializations.push(quote! {
                    #enum_name::#variant_name => usize::serialize(&#i)
                });
                deserializations.push(quote! {
                    #i => Ok(#enum_name::#variant_name)
                });
                serialized_sizes.push(quote! {
                    #enum_name::#variant_name => 1
                });
            } else {
                serializations.push(quote! {
                    #enum_name::#variant_name(val) => {
                        let mut temp = vec![];
                        temp.extend(usize::serialize(&#i));
                        temp.extend(#ty::serialize(val));
                        temp
                    }
                });
                deserializations.push(quote! {
                    #i => Ok(#enum_name::#variant_name(#ty::deserialize(felts, offset + 1)?))
                });
                // +1 because we have to handle the variant index also.
                serialized_sizes.push(quote! {
                    #enum_name::#variant_name(val) => #ty::serialized_size(val) + 1
                })
            }
        }

        deserializations.push(quote! {
           _ => panic!("Index not handle for {}", #name_str)
        });

        quote! {
            impl CairoType for #enum_name {
                type RustType = #enum_name;

                const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;

                #[inline]
                fn serialized_size(rust: &Self::RustType) -> usize {
                    match rust {
                        #(#serialized_sizes),*
                    }
                }

                fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {

                    match rust {
                        #(#serializations),*
                    }
                }

                fn deserialize(felts: &[FieldElement], offset: usize) -> cairo_types::Result<Self::RustType> {
                    let index:u128 = felts[offset].try_into().unwrap();
                    match index as usize {
                        #(#deserializations),*
                    }

                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Expandable;
    use cairo_type_parser::{abi_type::AbiType, CairoEnum};
    use proc_macro2::TokenStream;
    use quote::quote;

    #[test]
    fn test_decl_basic() {
        let ce = CairoEnum {
            name: AbiType::Basic("MyEnum".to_string()),
            variants: vec![
                (
                    "V1".to_string(),
                    AbiType::Basic("core::felt252".to_string()),
                ),
                (
                    "V2".to_string(),
                    AbiType::Basic("core::felt252".to_string()),
                ),
            ],
        };
        let te1 = ce.expand_decl();
        let tes1: TokenStream = quote!(
            pub enum MyEnum {
                V1(starknet::core::types::FieldElement),
                V2(starknet::core::types::FieldElement),
            }
        );

        assert_eq!(te1.to_string(), tes1.to_string());
    }
    #[test]
    fn test_impl_basic() {
        let ce = CairoEnum {
            name: AbiType::Basic("MyEnum".to_string()),
            variants: vec![
                (
                    "V1".to_string(),
                    AbiType::Basic("core::felt252".to_string()),
                ),
                (
                    "V2".to_string(),
                    AbiType::Basic("core::felt252".to_string()),
                ),
            ],
        };

        let te2 = ce.expand_impl();
        let tes2: TokenStream = quote!(
            impl CairoType for MyEnum{
            type RustType = MyEnum;
            const SERIALIZED_SIZE : Option <usize> = None ;

            # [inline]
            fn serialized_size (rust : &Self::RustType) -> usize {
                 match rust {
                    MyEnum::V1 (val) => < _ as CairoType >::serialized_size(val) ,
                    MyEnum::V2 (val) => < _ as CairoType >::serialized_size(val)
                } }
            fn serialize (rust : & Self :: RustType) -> Vec < FieldElement > {
                match rust {
                    MyEnum::V1 (val) => < _ as CairoType >::serialize(val) ,
                    MyEnum::V2 (val) => < _ as CairoType >::serialize(val) }
                }
            fn deserialize (felts : & [FieldElement] , offset : usize) -> Result < Self :: RustType > {
                if condition_for_variant (V1) {
                    return Ok (MyEnum :: V1 (< _ as CairoType > :: deserialize (felts , offset) ?)) ;
                } if condition_for_variant (V2) {
                    return Ok (MyEnum :: V2 (< _ as CairoType > :: deserialize (felts , offset) ?)) ;
                } Err ("Failed to deserialize variant.".into())}
            }
        );

        assert_eq!(te2.to_string(), tes2.to_string())
    }

    #[test]
    fn test_decl_tuple() {
        let ce = CairoEnum {
            name: AbiType::Basic("MyEnum".to_string()),
            variants: vec![
                (
                    "V1".to_string(),
                    AbiType::Tuple(vec![
                        AbiType::Basic("core::felt252".to_string()),
                        AbiType::Basic("core::felt252".to_string()),
                    ]),
                ),
                (
                    "V2".to_string(),
                    AbiType::Basic("core::felt252".to_string()),
                ),
            ],
        };

        let te1 = ce.expand_decl();
        let tes1: TokenStream = quote!(
            pub enum MyEnum {
                V1(
                    (
                        starknet::core::types::FieldElement,
                        starknet::core::types::FieldElement,
                    ),
                ),
                V2(starknet::core::types::FieldElement),
            }
        );

        assert_eq!(te1.to_string(), tes1.to_string());
    }
}
