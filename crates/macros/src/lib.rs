use proc_macro::TokenStream;

mod abigen;
use abigen::abigen_internal;

#[proc_macro]
pub fn abigen(input: TokenStream) -> TokenStream {
    abigen_internal(input)
}
