use crate::expand::utils::{str_to_ident, str_to_type};
use crate::Expandable;
use cairo_type_parser::{CairoFunction, CairoFunctionStateMutability};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl Expandable for CairoFunction {
    fn expand_decl(&self) -> TokenStream2 {
        let func_name = str_to_ident(&self.name.get_type_name_only());

        let mut inputs: Vec<TokenStream2> = vec![];
        let mut outputs: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.inputs {
            let name = str_to_ident(&name);
            let ty = str_to_type(&abi_type.to_rust_type());
            inputs.push(quote!(#name:#ty));
        }

        for (abi_type) in &self.outputs {
            let ty = str_to_type(&abi_type.to_rust_type());
            outputs.push(quote!(#ty));
        }

        let final_outputs = if outputs.len() > 0 {
            quote! {
                -> #(#outputs),*
            }
        } else {
            quote!()
        };

        let mutability = match &self.state_mutability {
            CairoFunctionStateMutability::View => quote!(),
            CairoFunctionStateMutability::External => quote!("pub"),
        };

        quote! {
            #mutability fn #func_name(
                #(#inputs),*
            ) #final_outputs
        }
    }
    fn expand_impl(&self) -> TokenStream2 {
        quote!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Expandable;
    use cairo_type_parser::{abi_type::AbiType, CairoFunction, CairoFunctionStateMutability};
    use proc_macro2::TokenStream;
    use quote::quote;

    #[test]
    fn test_decl_basic() {
        let cf = CairoFunction {
            name: AbiType::Basic("my_func".to_string()),
            state_mutability: CairoFunctionStateMutability::View,
            inputs: vec![
                (
                    "v1".to_string(),
                    AbiType::Basic("core::felt252".to_string()),
                ),
                (
                    "v2".to_string(),
                    AbiType::Basic("core::felt252".to_string()),
                ),
            ],
            outputs: vec![AbiType::Basic("core::felt252".to_string())],
        };
        let te1 = cf.expand_decl();
        let tef1: TokenStream = quote!(
           fn my_func(v1:starknet::core::types::FieldElement, v2:starknet::core::types::FieldElement) -> starknet::core::types::FieldElement
        );

        assert_eq!(te1.to_string(), tef1.to_string());
    }
}
