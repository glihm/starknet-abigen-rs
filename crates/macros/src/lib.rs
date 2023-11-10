use proc_macro::TokenStream;

mod abigen;
use abigen::abigen_internal;
use abigen::legacy::abigen_internal_legacy;

#[proc_macro]
pub fn abigen(input: TokenStream) -> TokenStream {
    abigen_internal(input)
}

#[proc_macro]
pub fn abigen_legacy(input: TokenStream) -> TokenStream {
    abigen_internal_legacy(input)
}
