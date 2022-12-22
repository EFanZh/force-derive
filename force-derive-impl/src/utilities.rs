use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn parse_derive_input(input: TokenStream) -> Result<DeriveInput, TokenStream> {
    syn::parse2(input).map_err(|error| error.to_compile_error())
}
