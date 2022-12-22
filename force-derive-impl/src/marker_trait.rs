use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn derive_marker_trait(input: DeriveInput, marker: TokenStream) -> TokenStream {
    let ty = input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    quote::quote! {
        #[automatically_derived]
        impl #impl_generics #marker for #ty #type_generics #where_clause {}
    }
}
