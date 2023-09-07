use syn::{parse_str, Ident, Type};

///
pub fn str_to_ident(str_in: &str) -> Ident {
    Ident::new(
        &str_in,
        proc_macro2::Span::call_site()
    )
}

///
pub fn str_to_type(str_in: &str) -> Type {
    syn::parse_str(&str_in)
        .expect(&format!("Can't convert {} to syn::Type", str_in))
}
