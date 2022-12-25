use proc_macro::TokenStream;
use syn::DeriveInput;

// Clone
// Copy
// Debug
// Default
// Eq
// Hash
// Ord
// PartialEq
// PartialOrd

mod clone;
mod copy;
mod debug;
mod default;
mod eq;
mod marker_trait;
mod partial_eq;
mod utilities;

fn derive_with(
    input: TokenStream,
    f: impl FnOnce(DeriveInput) -> proc_macro2::TokenStream,
) -> TokenStream {
    match utilities::parse_derive_input(input.into()) {
        Ok(input) => f(input).into(),
        Err(error) => error.into(),
    }
}

fn try_derive_with(
    input: TokenStream,
    f: impl FnOnce(DeriveInput) -> syn::Result<proc_macro2::TokenStream>,
) -> TokenStream {
    match utilities::parse_derive_input(input.into()) {
        Ok(input) => match f(input) {
            Ok(output) => output,
            Err(error) => error.into_compile_error(),
        }
        .into(),
        Err(error) => error.into(),
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

#[proc_macro_derive(PartialEq)]
pub fn derive_partial_eq(input: TokenStream) -> TokenStream {
    try_derive_with(input, partial_eq::derive_partial_eq)
}
