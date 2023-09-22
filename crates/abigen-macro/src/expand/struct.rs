use crate::expand::utils::{str_to_ident, str_to_type};
use crate::Expandable;

use cairo_type_parser::abi_types::AbiType;
use cairo_type_parser::CairoStruct;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl Expandable for CairoStruct {
    fn expand_decl(&self) -> TokenStream2 {
        let struct_name = str_to_ident(&self.get_name());

        let mut members: Vec<TokenStream2> = vec![];
        for (name, abitype) in &self.members {
            let name = str_to_ident(name);
            let ty = str_to_type(&abitype.to_rust_type());

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
        // let struct_name = str_to_ident(&self.get_name());

        // let mut sizes: Vec<TokenStream2> = vec![];
        // let mut sers: Vec<TokenStream2> = vec![];
        // let mut desers: Vec<TokenStream2> = vec![];
        // let mut names: Vec<TokenStream2> = vec![];

        // let mut is_first = true;
        // for (name, abitype) in &self.members {
        //     let name = str_to_ident(name);
        //     names.push(quote!(#name));

        //     let ty = str_to_type(&member.to_rust_type_path());

        //     // Tuples type used as rust type path must be surrounded
        //     // by LT/GT.
        //     let ty_punctuated = match member {
        //         AbiType::Tuple(_) => quote!(<#ty>),
        //         _ => quote!(#ty),
        //     };

        //     if is_first {
        //         sizes.push(quote!(#ty_punctuated::serialized_size(&rust.#name)));
        //         is_first = false;
        //     } else {
        //         sizes.push(quote!(+ #ty_punctuated::serialized_size(&rust.#name)));
        //     }

        //     sers.push(quote!(out.extend(#ty_punctuated::serialize(&rust.#name));));

        //     desers.push(quote! {
        //         let #name = #ty_punctuated::deserialize(felts, offset)?;
        //         offset += #ty_punctuated::serialized_size(&#name);
        //     });
        // }

        // quote! {
        //     impl cairo_types::CairoType for #struct_name {
        //         type RustType = Self;

        //         const SERIALIZED_SIZE: std::option::Option<usize> = None;

        //         #[inline]
        //         fn serialized_size(rust: &Self::RustType) -> usize {
        //             #(#sizes) *
        //         }

        //         fn serialize(rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
        //             let mut out: Vec<starknet::core::types::FieldElement> = vec![];
        //             #(#sers)*
        //             out
        //         }

        //         fn deserialize(felts: &[starknet::core::types::FieldElement], offset: usize) -> cairo_types::Result<Self::RustType> {
        //             let mut offset = offset;
        //             #(#desers)*
        //             Ok(#struct_name {
        //                 #(#names),*
        //             })
        //         }
        //     }
        // }

        quote!()
    }
}
