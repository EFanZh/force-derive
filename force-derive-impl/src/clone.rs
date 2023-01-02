use crate::utilities;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Fields, Generics, Ident, Index};

fn derive_with(ty: Ident, generics: Generics, body: TokenStream) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote::quote! {
        #[automatically_derived]
        impl #impl_generics ::core::clone::Clone for #ty #type_generics
        #where_clause
        {
            fn clone(&self) -> Self {
                #body
            }
        }
    }
}

pub fn derive_clone(input: DeriveInput) -> TokenStream {
    let clone = quote::quote!(::core::clone::Clone::clone);

    derive_with(
        input.ident,
        input.generics,
        match input.data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields) => {
                    let fields = fields.named.into_iter().map(|field| field.ident.unwrap());

                    quote::quote! { Self { #(#fields: #clone(&self.#fields),)* } }
                }
                Fields::Unnamed(fields) => {
                    let fields = (0..fields.unnamed.len()).map(Index::from);

                    quote::quote! { Self(#(#clone(&self.#fields),)*) }
                }
                Fields::Unit => quote::quote! { Self },
            },
            Data::Enum(data_enum) => {
                let variants = data_enum.variants;

                if variants.is_empty() {
                    quote::quote! { match *self {} }
                } else {
                    let arms = variants.into_iter().map(|variant| {
                        let variant_name = variant.ident;

                        match variant.fields {
                            Fields::Named(fields) => {
                                let pattern_fields = fields
                                    .named
                                    .iter()
                                    .map(|field| field.ident.as_ref().unwrap());

                                    let expression_fields = pattern_fields.clone();
                                    let expression_variables = pattern_fields.clone();

                                quote::quote! {
                                    Self::#variant_name { #(#pattern_fields,)* } => Self::#variant_name { #(#expression_fields: #clone(#expression_variables),)* }
                                }
                            }
                            Fields::Unnamed(fields) => {
                                let fields = utilities::get_field_identifiers(fields.unnamed.len())
                                    .collect::<Vec<_>>();

                                quote::quote! {
                                    Self::#variant_name(#(#fields,)*) => Self::#variant_name(#(#clone(#fields),)*)
                                }
                            }
                            Fields::Unit => quote::quote! {
                                Self::#variant_name => Self::#variant_name
                            },
                        }
                    });

                    quote::quote! {
                        match self {
                            #(#arms,)*
                        }
                    }
                }
            }
            Data::Union(_) => quote::quote!(*self),
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::utilities;

    #[test]
    fn test_derive_clone() {
        let test_cases = [
            // Empty struct.
            (
                quote::quote! {
                    struct Foo {}
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::clone::Clone for Foo {
                        fn clone(&self) -> Self {
                            Self {}
                        }
                    }
                },
            ),
            // Struct with a single field.
            (
                quote::quote! {
                    struct Foo<T> {
                        foo: Rc<T>,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::clone::Clone for Foo<T> {
                        fn clone(&self) -> Self {
                            Self {
                                foo: ::core::clone::Clone::clone(&self.foo),
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
                        foo: Rc<T>,
                        bar: PhantomData<T>,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::clone::Clone for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn clone(&self) -> Self {
                            Self {
                                foo: ::core::clone::Clone::clone(&self.foo),
                                bar: ::core::clone::Clone::clone(&self.bar),
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
                    impl ::core::clone::Clone for Foo {
                        fn clone(&self) -> Self {
                            Self()
                        }
                    }
                },
            ),
            // Tuple with a single field.
            (
                quote::quote! {
                    struct Foo<T>(Rc<T>);
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::clone::Clone for Foo<T> {
                        fn clone(&self) -> Self {
                            Self(::core::clone::Clone::clone(&self.0),)
                        }
                    }
                },
            ),
            // Tuple with two fields and generic constraints.
            (
                quote::quote! {
                    struct Foo<T>(Rc<T>, PhantomData<T>)
                    where
                        u32: Copy;
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::clone::Clone for Foo<T>
                    where
                        u32: Copy
                    {
                        fn clone(&self) -> Self {
                            Self(
                                ::core::clone::Clone::clone(&self.0),
                                ::core::clone::Clone::clone(&self.1),
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
                    impl ::core::clone::Clone for Foo {
                        fn clone(&self) -> Self {
                            Self
                        }
                    }
                },
            ),
            // Empty enum.
            (
                quote::quote! {
                    enum Foo {}
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::clone::Clone for Foo {
                        fn clone(&self) -> Self {
                            match *self {}
                        }
                    }
                },
            ),
            // Enum with a single variant.
            (
                quote::quote! {
                    enum Foo<T> {
                        Tuple1(Rc<T>),
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::clone::Clone for Foo<T> {
                        fn clone(&self) -> Self {
                            match self {
                                Self::Tuple1(field_0,) => Self::Tuple1(::core::clone::Clone::clone(field_0),),
                            }
                        }
                    }
                },
            ),
            // Enum.
            (
                quote::quote! {
                    enum Foo<T>
                    where
                        u32: Copy,
                    {
                        Struct0 {},
                        Struct1 { foo: Rc<T> },
                        Struct2 { foo: Rc<T>, bar: PhantomData<T> },
                        Tuple0(),
                        Tuple1(Rc<T>),
                        Tuple2(Rc<T>, PhantomData<T>),
                        Unit,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::clone::Clone for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn clone(&self) -> Self {
                            match self {
                                Self::Struct0 {} => Self::Struct0 {},
                                Self::Struct1 { foo, } => Self::Struct1 { foo: ::core::clone::Clone::clone(foo), },
                                Self::Struct2 { foo, bar, } => Self::Struct2 {
                                    foo: ::core::clone::Clone::clone(foo),
                                    bar: ::core::clone::Clone::clone(bar),
                                },
                                Self::Tuple0() => Self::Tuple0(),
                                Self::Tuple1(field_0,) => Self::Tuple1(::core::clone::Clone::clone(field_0),),
                                Self::Tuple2(field_0, field_1,) => Self::Tuple2(
                                    ::core::clone::Clone::clone(field_0),
                                    ::core::clone::Clone::clone(field_1),
                                ),
                                Self::Unit => Self::Unit,
                            }
                        }
                    }
                },
            ),
            // Union.
            (
                quote::quote! {
                    union Foo {
                        foo: u32,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::clone::Clone for Foo {
                        fn clone(&self) -> Self {
                            *self
                        }
                    }
                },
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                super::derive_clone(utilities::parse_derive_input(input).unwrap()).to_string(),
                expected.to_string(),
            );
        }
    }
}
