use crate::expand::utils::{str_to_ident, str_to_type};
use cairo_type_parser::abi_types::{AbiType, AbiTypeAny};
use cairo_type_parser::CairoEnum;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

pub fn impl_with_gentys_tokens(entity_name: &Ident, gentys: &Vec<Ident>) -> TokenStream2 {
    let gentys_rust: Vec<Ident> = gentys
        .iter()
        .map(|g| str_to_ident(format!("R{}", g.to_string()).as_str())).collect();

    let mut tokens = vec![];

    tokens.push(quote! {
        impl<#(#gentys),* , #(#gentys_rust),*> cairo_types::CairoType for #entity_name<#(#gentys),*>
        where
    });

    for (i, g) in gentys.iter().enumerate() {
        let gr = &gentys_rust[i];
        tokens.push(quote!(#g: CairoType<RustType = #gr>,));
    }

    quote!(#(#tokens)*)
}

pub fn rust_associated_type_gentys_tokens(entity_name: &Ident, gentys: &Vec<Ident>) -> TokenStream2 {
    let gentys_rust: Vec<Ident> = gentys
        .iter()
        .map(|g| str_to_ident(format!("R{}", g.to_string()).as_str())).collect();

    quote!(type RustType = #entity_name<#(#gentys_rust),*>;)
}
