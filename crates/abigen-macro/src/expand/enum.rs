use crate::expand::utils::{str_to_ident, str_to_type};
use crate::Expandable;
use cairo_type_parser::CairoEnum;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl Expandable for CairoEnum {
    fn expand_decl(&self) -> TokenStream2 {
        let enum_name = str_to_ident(&self.name.get_type_name_only());

        let mut variants: Vec<TokenStream2> = vec![];

        for (name, abi_type) in &self.variants {
            let name = str_to_ident(&name);
            let ty = str_to_type(&abi_type.to_rust_type());

            variants.push(quote!(#name(#ty)));
        }

        quote! {
            pub enum #enum_name {
                #(#variants),*
            }
        }
    }
    fn expand_impl(&self) -> TokenStream2 {
        let enum_name = str_to_ident(&self.name.get_type_name_only());

        let mut variants: Vec<TokenStream2> = vec![];

        let serialized_sizes: Vec<_> = self
            .variants
            .iter()
            .map(|variant| {
                let variant_name = str_to_ident(&variant.0);
                quote! {
                    #enum_name::#variant_name(val) => <_ as CairoType>::serialized_size(val)
                }
            })
            .collect();

        let serializations: Vec<_> = self
            .variants
            .iter()
            .map(|variant| {
                let variant_name = str_to_ident(&variant.0);
                quote! {
                    #enum_name::#variant_name(val) => <_ as CairoType>::serialize(val)
                }
            })
            .collect();

        let deserializations: Vec<_> = self.variants.iter().map(|variant| {
                let variant_name = str_to_ident(&variant.0);
                quote! {
                    if condition_for_variant(#variant_name) {
                        return Ok(#enum_name::#variant_name(<_ as CairoType>::deserialize(felts, offset)?));
                    }
                }
            }).collect();

        quote! {
            impl CairoType for #enum_name {
                type RustType = #enum_name;

                const SERIALIZED_SIZE: Option<usize> = None;

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

                fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
                    #(#deserializations)*
                    Err("Failed to deserialize variant.".into())
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
    fn test_1() {
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

        let te2 = ce.expand_impl();
        let tes2: TokenStream = quote!(
            impl CairoType for MyEnum
            { type RustType = MyEnum ;
              const SERIALIZED_SIZE : Option < usize > = None ;
            # [inline]
            fn serialized_size (rust : & Self :: RustType) -> usize {
                 match rust {
                    MyEnum :: V1 (val) => < _ as CairoType > :: serialized_size (val) ,
                    MyEnum :: V2 (val) => < _ as CairoType > :: serialized_size (val)
                } }
            fn serialize (rust : & Self :: RustType) -> Vec < FieldElement > {
                match rust {
                    MyEnum :: V1 (val) => < _ as CairoType > :: serialize (val) ,
                    MyEnum :: V2 (val) => < _ as CairoType > :: serialize (val) }
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
}
