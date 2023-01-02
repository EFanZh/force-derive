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
            // Empty struct.
            (
                quote::quote! {
                    struct Foo {}
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::default::Default for Foo {
                        fn default() -> Self {
                            Self {}
                        }
                    }
                },
            ),
            // Struct with a single field.
            (
                quote::quote! {
                    struct Foo<T> {
                        foo: Vec<T>,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T> {
                        fn default() -> Self {
                            Self {
                                foo: ::core::default::Default::default(),
                            }
                        }
                    }
                },
            ),
            // Struct with two fields and generic constraints.
            (
                quote::quote! {
                    struct Foo<T>
                    where
                        u32: Copy,
                    {
                        foo: Vec<T>,
                        bar: PhantomData<T>,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn default() -> Self {
                            Self {
                                foo: ::core::default::Default::default(),
                                bar: ::core::default::Default::default(),
                            }
                        }
                    }
                },
            ),
            // Empty tuple.
            (
                quote::quote! {
                    struct Foo();
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::default::Default for Foo {
                        fn default() -> Self {
                            Self()
                        }
                    }
                },
            ),
            // Tuple with a single field.
            (
                quote::quote! {
                    struct Foo<T>(Vec<T>);
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T> {
                        fn default() -> Self {
                            Self(::core::default::Default::default(),)
                        }
                    }
                },
            ),
            // Tuple with two fields and generic constraints.
            (
                quote::quote! {
                    struct Foo<T>(Vec<T>, PhantomData<T>)
                    where
                        u32: Copy;
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T>
                    where
                        u32: Copy
                    {
                        fn default() -> Self {
                            Self(
                                ::core::default::Default::default(),
                                ::core::default::Default::default(),
                            )
                        }
                    }
                },
            ),
            // Unit.
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
            // Enum with a single variant.
            (
                quote::quote! {
                    enum Foo<T> {
                        #[default]
                        Tuple1(Vec<T>),
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T> {
                        fn default() -> Self {
                            Self::Tuple1(::core::default::Default::default(),)
                        }
                    }
                },
            ),
            // Enum with empty struct as default.
            (
                quote::quote! {
                    enum Foo<T>
                    where
                        u32: Copy,
                    {
                        #[default]
                        Struct0 {},
                        Struct1 { foo: Vec<T> },
                        Struct2 { foo: Vec<T>, bar: PhantomData<T> },
                        Tuple0(),
                        Tuple1(Vec<T>),
                        Tuple2(Vec<T>, PhantomData<T>),
                        Unit,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn default() -> Self {
                            Self::Struct0 {}
                        }
                    }
                },
            ),
            // Enum with struct with a single field as default.
            (
                quote::quote! {
                    enum Foo<T>
                    where
                        u32: Copy,
                    {
                        Struct0 {},
                        #[default]
                        Struct1 { foo: Vec<T> },
                        Struct2 { foo: Vec<T>, bar: PhantomData<T> },
                        Tuple0(),
                        Tuple1(Vec<T>),
                        Tuple2(Vec<T>, PhantomData<T>),
                        Unit,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn default() -> Self {
                            Self::Struct1 {
                                foo: ::core::default::Default::default(),
                            }
                        }
                    }
                },
            ),
            // Enum with struct with two fields as default.
            (
                quote::quote! {
                    enum Foo<T>
                    where
                        u32: Copy,
                    {
                        Struct0 {},
                        Struct1 { foo: Vec<T> },
                        #[default]
                        Struct2 { foo: Vec<T>, bar: PhantomData<T> },
                        Tuple0(),
                        Tuple1(Vec<T>),
                        Tuple2(Vec<T>, PhantomData<T>),
                        Unit,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn default() -> Self {
                            Self::Struct2 {
                                foo: ::core::default::Default::default(),
                                bar: ::core::default::Default::default(),
                            }
                        }
                    }
                },
            ),
            // Enum with empty tuple as default.
            (
                quote::quote! {
                    enum Foo<T>
                    where
                        u32: Copy,
                    {
                        Struct0 {},
                        Struct1 { foo: Vec<T> },
                        Struct2 { foo: Vec<T>, bar: PhantomData<T> },
                        #[default]
                        Tuple0(),
                        Tuple1(Vec<T>),
                        Tuple2(Vec<T>, PhantomData<T>),
                        Unit,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn default() -> Self {
                            Self::Tuple0()
                        }
                    }
                },
            ),
            // Enum with struct with a single field as default.
            (
                quote::quote! {
                    enum Foo<T>
                    where
                        u32: Copy,
                    {
                        Struct0 {},
                        Struct1 { foo: Vec<T> },
                        Struct2 { foo: Vec<T>, bar: PhantomData<T> },
                        Tuple0(),
                        #[default]
                        Tuple1(Vec<T>),
                        Tuple2(Vec<T>, PhantomData<T>),
                        Unit,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn default() -> Self {
                            Self::Tuple1(::core::default::Default::default(),)
                        }
                    }
                },
            ),
            // Enum with struct with two fields as default.
            (
                quote::quote! {
                    enum Foo<T>
                    where
                        u32: Copy,
                    {
                        Struct0 {},
                        Struct1 { foo: Vec<T> },
                        Struct2 { foo: Vec<T>, bar: PhantomData<T> },
                        Tuple0(),
                        Tuple1(Vec<T>),
                        #[default]
                        Tuple2(Vec<T>, PhantomData<T>),
                        Unit,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn default() -> Self {
                            Self::Tuple2(
                                ::core::default::Default::default(),
                                ::core::default::Default::default(),
                            )
                        }
                    }
                },
            ),
            // Enum with empty struct as default.
            (
                quote::quote! {
                    enum Foo<T>
                    where
                        u32: Copy,
                    {
                        Struct0 {},
                        Struct1 { foo: Vec<T> },
                        Struct2 { foo: Vec<T>, bar: PhantomData<T> },
                        Tuple0(),
                        Tuple1(Vec<T>),
                        Tuple2(Vec<T>, PhantomData<T>),
                        #[default]
                        Unit,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::default::Default for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn default() -> Self {
                            Self::Unit
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
    fn test_derive_default_wrong_default_attribute() {
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
