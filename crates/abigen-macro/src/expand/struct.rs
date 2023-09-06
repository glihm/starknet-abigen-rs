// use proc_macro2::TokenStream as TokenStream2;
// use syn::{
//     braced,
//     ext::IdentExt,
//     parenthesized,
//     parse::{Error, Parse, ParseStream, Result},
//     punctuated::Punctuated,
//     Ident, LitInt, LitStr, Token, Type,
//     parse_macro_input,
// };
// use std::fs;
// use std::path::Path;
// use serde_json;
// use starknet::core::types::*;
// use starknet::core::types::contract::*;
// use quote::quote;
// use starknet::accounts::Call;
// use std::collections::HashMap;

// pub(super) fn expand(s: AbiStruct) -> TokenStream2 {
//     let struct_name = str_to_ident(&s.name);

//     let mut fields: Vec<TokenStream2> = vec![];
//     for member in s.members {
//         let name = str_to_ident(&member.name);
//         let ty = convert_abi_type_to_rust_type(&member.r#type);
//         fields.push(quote!(#name: #ty));
//     }

//     tokens.push(quote! {
//         #[derive(Debug)]
//         pub struct #struct_name {
//             #(#fields),*
//         }
//     });    
// }
