use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Fields, Generics, Ident, Index, Variant};

fn derive_with(ty: Ident, generics: Generics, body: TokenStream) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote::quote! {
        #[automatically_derived]
        impl #impl_generics ::core::hash::Hash for #ty #type_generics
        #where_clause
        {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                #body
            }
        }
    }
}

fn hash_variant(hash: &TokenStream, variant: &Variant) -> TokenStream {
    let variant_name = &variant.ident;

    match &variant.fields {
        Fields::Named(fields) => {
            let field_names = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap());

            let field_variables = (0..fields.named.len())
                .map(|i| quote::format_ident!("field_{}", i))
                .collect::<Vec<_>>();

            quote::quote! {
                Self::#variant_name { #(#field_names: #field_variables,)* } => {
                    #(#hash(#field_variables, state);)*
                }
            }
        }
        Fields::Unnamed(fields) => {
            let fields = (0..fields.unnamed.len())
                .map(|i| quote::format_ident!("field_{}", i))
                .collect::<Vec<_>>();

            quote::quote! {
                Self::#variant_name(#(#fields,)*) => {
                    #(#hash(#fields, state);)*
                }
            }
        }
        Fields::Unit => quote::quote! { Self::#variant_name => {} },
    }
}

pub fn derive_hash(input: DeriveInput) -> syn::Result<TokenStream> {
    let span = input.ident.span();
    let hash = quote::quote!(::core::hash::Hash::hash);

    Ok(derive_with(
        input.ident,
        input.generics,
        match input.data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields) => {
                    let fields = fields.named.into_iter().map(|field| field.ident.unwrap());

                    quote::quote! { #(#hash(&self.#fields, state);)* }
                }
                Fields::Unnamed(fields) => {
                    let fields = (0..fields.unnamed.len()).map(Index::from);

                    quote::quote! { #(#hash(&self.#fields, state);)* }
                }
                Fields::Unit => quote::quote! {},
            },
            Data::Enum(data_enum) => {
                let variants = data_enum.variants;

                if let Some(first) = variants.first() {
                    if variants.len() == 1 {
                        let arm = hash_variant(&hash, first);

                        quote::quote! {
                            match self {
                                #arm,
                            }
                        }
                    } else {
                        let arms = variants.iter().map(|variant| hash_variant(&hash, variant));

                        quote::quote! {
                            #hash(&::core::mem::discriminant(self), state);

                            match self {
                                #(#arms,)*
                            }
                        }
                    }
                } else {
                    quote::quote!(match *self {})
                }
            }
            Data::Union(_) => {
                return Err(syn::Error::new(span, "Cannot derive `Hash` on a `union`."))
            }
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::utilities;

    #[test]
    fn test_derive_hash() {
        let test_cases = [
            // Named struct type.
            (
                quote::quote! {
                    struct Foo {
                        field_1: Type1,
                        field_2: Type2
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::hash::Hash for Foo {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            ::core::hash::Hash::hash(&self.field_1, state);
                            ::core::hash::Hash::hash(&self.field_2, state);
                        }
                    }
                },
            ),
            // Generic named struct type.
            (
                quote::quote! {
                    struct Foo<T, U>
                    where
                        T: Trait1,
                        U: Trait2,
                    {
                        field_1: Type1,
                        field_2: Type2<T>,
                        field_3: Type3<U>,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T, U> ::core::hash::Hash for Foo<T, U>
                    where
                        T: Trait1,
                        U: Trait2,
                    {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            ::core::hash::Hash::hash(&self.field_1, state);
                            ::core::hash::Hash::hash(&self.field_2, state);
                            ::core::hash::Hash::hash(&self.field_3, state);
                        }
                    }
                },
            ),
            // Tuple struct type.
            (
                quote::quote! {
                    struct Foo(X, Y);
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::hash::Hash for Foo {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            ::core::hash::Hash::hash(&self.0, state);
                            ::core::hash::Hash::hash(&self.1, state);
                        }
                    }
                },
            ),
            // Unit struct type.
            (
                quote::quote! {
                    struct Foo;
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::hash::Hash for Foo {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {}
                    }
                },
            ),
            // Empty enum type.
            (
                quote::quote! {
                    enum Foo {}
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::hash::Hash for Foo {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            match *self {}
                        }
                    }
                },
            ),
            // Single enum type.
            (
                quote::quote! {
                    enum Foo {
                        X,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::hash::Hash for Foo {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            match self {
                                Self::X => {},
                            }
                        }
                    }
                },
            ),
            // Enum type.
            (
                quote::quote! {
                    enum Foo {
                        X,
                        Y(A, B),
                        Z {
                            a: A,
                            b: B,
                        }
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::hash::Hash for Foo {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            ::core::hash::Hash::hash(&::core::mem::discriminant(self), state);

                            match self {
                                Self::X => {},
                                Self::Y(field_0, field_1,) => {
                                    ::core::hash::Hash::hash(field_0, state);
                                    ::core::hash::Hash::hash(field_1, state);
                                },
                                Self::Z {
                                    a: field_0,
                                    b: field_1,
                                } => {
                                    ::core::hash::Hash::hash(field_0, state);
                                    ::core::hash::Hash::hash(field_1, state);
                                },
                            }
                        }
                    }
                },
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                super::derive_hash(utilities::parse_derive_input(input).unwrap())
                    .unwrap()
                    .to_string(),
                expected.to_string(),
            );
        }
    }
}
