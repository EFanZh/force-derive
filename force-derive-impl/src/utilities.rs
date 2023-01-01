use proc_macro2::{Ident, TokenStream};
use syn::DeriveInput;

pub fn parse_derive_input(input: TokenStream) -> Result<DeriveInput, TokenStream> {
    syn::parse2(input).map_err(|error| error.to_compile_error())
}

pub fn get_field_identifiers(n: usize) -> impl Iterator<Item = Ident> {
    (0..n).map(|i| quote::format_ident!("field_{}", i))
}
