use proc_macro::TokenStream;
use syn::DeriveInput;

// TODO:
//
// - [x] Clone
// - [x] Copy
// - [x] Debug
// - [x] Default
// - [x] Eq
// - [x] Hash
// - [ ] Ord
// - [x] PartialEq
// - [ ] PartialOrd
// - Error span.
// - Variable name conflict.
// - `?Sized` field.

mod clone;
mod copy;
mod debug;
mod default;
mod eq;
mod hash;
mod marker_trait;
mod partial_eq;
mod partial_ord;
mod utilities;

fn parse_derive_input(input: TokenStream) -> Result<DeriveInput, TokenStream> {
    utilities::parse_derive_input(input.into()).map_err(|error| error.into())
}

fn derive_with(input: TokenStream, f: impl FnOnce(DeriveInput) -> proc_macro2::TokenStream) -> TokenStream {
    match parse_derive_input(input) {
        Ok(input) => f(input).into(),
        Err(error) => error,
    }
}

fn flatten_result(result: syn::Result<proc_macro2::TokenStream>) -> TokenStream {
    result.unwrap_or_else(syn::Error::into_compile_error).into()
}

fn try_derive_with(
    input: TokenStream,
    f: impl FnOnce(DeriveInput) -> syn::Result<proc_macro2::TokenStream>,
) -> TokenStream {
    match parse_derive_input(input) {
        Ok(input) => flatten_result(f(input)),
        Err(error) => error,
    }
}

#[proc_macro_derive(Clone)]
pub fn derive_clone(input: TokenStream) -> TokenStream {
    derive_with(input, clone::derive_clone)
}

#[proc_macro_derive(Copy)]
pub fn derive_copy(input: TokenStream) -> TokenStream {
    derive_with(input, copy::derive_copy)
}

#[proc_macro_derive(Debug)]
pub fn derive_debug(input: TokenStream) -> TokenStream {
    try_derive_with(input, debug::derive_debug)
}

#[proc_macro_derive(Default, attributes(default))]
pub fn derive_default(input: TokenStream) -> TokenStream {
    try_derive_with(input, default::derive_default)
}

#[proc_macro_derive(Eq)]
pub fn derive_eq(input: TokenStream) -> TokenStream {
    derive_with(input, eq::derive_eq)
}

#[proc_macro_derive(Hash)]
pub fn derive_hash(input: TokenStream) -> TokenStream {
    try_derive_with(input, hash::derive_hash)
}

#[proc_macro_derive(PartialEq)]
pub fn derive_partial_eq(input: TokenStream) -> TokenStream {
    try_derive_with(input, partial_eq::derive_partial_eq)
}

#[proc_macro_derive(PartialOrd)]
pub fn derive_partial_ord(input: TokenStream) -> TokenStream {
    try_derive_with(input, partial_ord::derive_partial_ord)
}
