use proc_macro2::TokenStream;
use std::iter;
use syn::{Data, DeriveInput, Fields, Generics, Ident, Meta};

fn derive_with(ty: Ident, generics: Generics, body: TokenStream) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote::quote! {
        #[automatically_derived]
        impl #impl_generics ::core::default::Default for #ty #type_generics
        #where_clause
        {
            fn default() -> Self {
                #body
            }
        }
    }
}

pub fn derive_default(input: DeriveInput) -> syn::Result<TokenStream> {
    let default = quote::quote!(::core::default::Default::default());
    let span = input.ident.span();

    Ok(derive_with(
        input.ident,
        input.generics,
        match input.data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields) => {
                    let fields = fields.named.into_iter().map(|field| field.ident.unwrap());

                    quote::quote! { Self { #(#fields: #default,)* } }
                }
                Fields::Unnamed(fields) => {
                    let fields = iter::repeat(&default).take(fields.unnamed.len());

                    quote::quote! { Self(#(#fields,)*) }
                }
                Fields::Unit => quote::quote! { Self },
            },
            Data::Enum(data_enum) => {
                let mut default_variants_iter = data_enum.variants.into_iter().filter(|variant| {
                    variant.attrs.iter().any(|attr| {
                        attr.parse_meta().map_or(false, |meta| {
                            if let Meta::Path(path) = meta {
                                path.is_ident("default")
                            } else {
                                false
                            }
                        })
                    })
                });

                if let Some(variant) = default_variants_iter
                    .next()
                    .filter(|_| default_variants_iter.next().is_none())
                {
                    let variant_name = variant.ident;

                    match variant.fields {
                        Fields::Named(fields) => {
                            let fields = fields.named.into_iter().map(|field| field.ident.unwrap());

                            quote::quote! { Self::#variant_name { #(#fields: #default,)* } }
                        }
                        Fields::Unnamed(fields) => {
                            let fields = iter::repeat(&default).take(fields.unnamed.len());

                            quote::quote! { Self::#variant_name(#(#fields,)*) }
                        }
                        Fields::Unit => quote::quote! { Self::#variant_name },
                    }
                } else {
                    return Err(syn::Error::new(
                        span,
                        "Use a single `#[default]` attribute to mark a variant as the default one.",
                    ));
                }
            }
            Data::Union(_) => quote::quote!(*self),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::utilities;

    #[test]
    fn test_derive_default() {
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
                    impl ::core::default::Default for Foo {
                        fn default() -> Self {
                            Self { field_1: ::core::default::Default::default(), field_2: ::core::default::Default::default(), }
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
                    impl<T, U> ::core::default::Default for Foo<T, U>
                    where
                        T: Trait1,
                        U: Trait2,
                    {
                        fn default() -> Self {
                            Self {
                                field_1: ::core::default::Default::default(),
                                field_2: ::core::default::Default::default(),
                                field_3: ::core::default::Default::default(),
                            }
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
                    impl ::core::default::Default for Foo {
                        fn default() -> Self {
                            Self(::core::default::Default::default(), ::core::default::Default::default(),)
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
                    impl ::core::default::Default for Foo {
                        fn default() -> Self {
                            Self
                        }
                    }
                },
            ),
            // Single enum type.
            (
                quote::quote! {
                    enum Foo {
                        #[default]
                        X,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::default::Default for Foo {
                        fn default() -> Self {
                            Self::X
                        }
                    }
                },
            ),
            // Enum type 1.
            (
                quote::quote! {
                    enum Foo {
                        #[default]
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
                    impl ::core::default::Default for Foo {
                        fn default() -> Self {
                            Self::X
                        }
                    }
                },
            ),
            // Enum type 2.
            (
                quote::quote! {
                    enum Foo {
                        X,
                        #[default]
                        Y(A, B),
                        Z {
                            a: A,
                            b: B,
                        }
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::default::Default for Foo {
                        fn default() -> Self {
                            Self::Y(::core::default::Default::default(), ::core::default::Default::default(),)
                        }
                    }
                },
            ),
            // Enum type 3.
            (
                quote::quote! {
                    enum Foo {
                        X,
                        Y(A, B),
                        #[default]
                        Z {
                            a: A,
                            b: B,
                        }
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::default::Default for Foo {
                        fn default() -> Self {
                            Self::Z { a: ::core::default::Default::default(), b: ::core::default::Default::default(), }
                        }
                    }
                },
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                super::derive_default(utilities::parse_derive_input(input).unwrap())
                    .unwrap()
                    .to_string(),
                expected.to_string(),
            );
        }
    }

    #[test]
    fn test_derive_default_missing_default_attribute() {
        let test_cases = [
            quote::quote! {
                enum Foo {}
            },
            quote::quote! {
                enum Foo {
                    A,
                }
            },
            quote::quote! {
                enum Foo {
                    A,
                    B,
                }
            },
            quote::quote! {
                enum Foo {
                    #[default]
                    A,
                    #[default]
                    B,
                }
            },
        ];

        for input in test_cases {
            assert!(super::derive_default(utilities::parse_derive_input(input).unwrap()).is_err());
        }
    }
}
