use proc_macro2::{Span, TokenStream};
use syn::{Data, DeriveInput, Fields, Generics, Ident, Index, Variant};

fn unit_type() -> (TokenStream, TokenStream) {
    (quote::quote!(true), quote::quote!(false))
}

fn generate_variant(variant: &Variant) -> (TokenStream, TokenStream) {
    let variant_name = &variant.ident;

    match &variant.fields {
        Fields::Named(fields) => {
            let eq_pattern_self_fields = fields.named.iter().map(|field| field.ident.as_ref().unwrap());

            let self_variables = eq_pattern_self_fields
                .clone()
                .map(|field| quote::format_ident!("self_{}", field))
                .collect::<Vec<_>>();

            let eq_pattern_other_fields = eq_pattern_self_fields.clone();

            let other_variables = eq_pattern_self_fields
                .clone()
                .map(|field| quote::format_ident!("other_{}", field))
                .collect::<Vec<_>>();

            let ne_pattern_self_fields = eq_pattern_self_fields.clone();
            let ne_pattern_other_fields = eq_pattern_other_fields.clone();

            let (eq_expression, ne_expression) = if fields.named.is_empty() {
                (quote::quote!(true), quote::quote!(false))
            } else {
                (
                    quote::quote!(#(::core::cmp::PartialEq::eq(#self_variables, #other_variables))&&*),
                    quote::quote!(#(::core::cmp::PartialEq::ne(#self_variables, #other_variables))||*),
                )
            };

            (
                quote::quote! {
                    (
                        Self::#variant_name { #(#eq_pattern_self_fields: #self_variables,)* },
                        Self::#variant_name { #(#eq_pattern_other_fields: #other_variables,)* },
                    ) => #eq_expression
                },
                quote::quote! {
                    (
                        Self::#variant_name { #(#ne_pattern_self_fields: #self_variables,)* },
                        Self::#variant_name { #(#ne_pattern_other_fields: #other_variables,)* },
                    ) => #ne_expression
                },
            )
        }
        Fields::Unnamed(fields) => {
            let self_variables = (0..fields.unnamed.len())
                .map(|field| quote::format_ident!("self_{}", field))
                .collect::<Vec<_>>();

            let other_variables = (0..fields.unnamed.len())
                .map(|field| quote::format_ident!("other_{}", field))
                .collect::<Vec<_>>();

            let (eq_expression, ne_expression) = if fields.unnamed.is_empty() {
                (quote::quote!(true), quote::quote!(false))
            } else {
                (
                    quote::quote!(#(::core::cmp::PartialEq::eq(#self_variables, #other_variables))&&*),
                    quote::quote!(#(::core::cmp::PartialEq::ne(#self_variables, #other_variables))||*),
                )
            };

            (
                quote::quote! {
                    (
                        Self::#variant_name(#(#self_variables,)*),
                        Self::#variant_name(#(#other_variables,)*),
                    ) => #eq_expression
                },
                quote::quote! {
                    (
                        Self::#variant_name(#(#self_variables,)*),
                        Self::#variant_name(#(#other_variables,)*),
                    ) => #ne_expression
                },
            )
        }
        Fields::Unit => (
            quote::quote! { (Self::#variant_name, Self::#variant_name,) => true },
            quote::quote! { (Self::#variant_name, Self::#variant_name,) => false },
        ),
    }
}

fn generate_function_bodies(span: Span, data: Data) -> syn::Result<(TokenStream, TokenStream)> {
    Ok(match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                if fields.named.is_empty() {
                    unit_type()
                } else {
                    let eq_self_fields = fields.named.iter().map(|field| field.ident.as_ref().unwrap());
                    let eq_other_fields = eq_self_fields.clone();
                    let ne_self_fields = eq_self_fields.clone();
                    let ne_other_fields = eq_self_fields.clone();

                    (
                        quote::quote!(#(::core::cmp::PartialEq::eq(&self.#eq_self_fields, &other.#eq_other_fields))&&*),
                        quote::quote!(#(::core::cmp::PartialEq::ne(&self.#ne_self_fields, &other.#ne_other_fields))||*),
                    )
                }
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.is_empty() {
                    unit_type()
                } else {
                    let eq_self_fields = (0..fields.unnamed.len()).map(Index::from);
                    let eq_other_fields = eq_self_fields.clone();
                    let ne_self_fields = eq_self_fields.clone();
                    let ne_other_fields = eq_self_fields.clone();

                    (
                        quote::quote!(#(::core::cmp::PartialEq::eq(&self.#eq_self_fields, &other.#eq_other_fields))&&*),
                        quote::quote!(#(::core::cmp::PartialEq::ne(&self.#ne_self_fields, &other.#ne_other_fields))||*),
                    )
                }
            }
            Fields::Unit => unit_type(),
        },
        Data::Enum(data_enum) => {
            let variants = data_enum.variants;

            if let Some(first) = variants.first() {
                if variants.len() == 1 {
                    let (eq, ne) = generate_variant(first);

                    (
                        quote::quote! {
                            match (self, other) {
                                #eq,
                            }
                        },
                        quote::quote! {
                            match (self, other) {
                                #ne,
                            }
                        },
                    )
                } else {
                    let (eq, ne): (Vec<_>, Vec<_>) = variants.iter().map(generate_variant).unzip();

                    (
                        quote::quote! {
                            match (self, other) {
                                #(#eq,)*
                                _ => false,
                            }
                        },
                        quote::quote! {
                            match (self, other) {
                                #(#ne,)*
                                _ => true,
                            }
                        },
                    )
                }
            } else {
                (quote::quote! { match *self {} }, quote::quote! { match *self {} })
            }
        }
        Data::Union(_) => return Err(syn::Error::new(span, "Cannot derive `PartialEq` on a `union`.")),
    })
}

fn derive_with(ty: Ident, generics: Generics, eq_body: TokenStream, ne_body: TokenStream) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote::quote! {
        #[automatically_derived]
        impl #impl_generics ::core::cmp::PartialEq for #ty #type_generics
        #where_clause
        {
            fn eq(&self, other: &Self) -> bool {
                #eq_body
            }

            fn ne(&self, other: &Self) -> bool {
                #ne_body
            }
        }
    }
}

pub fn derive_partial_eq(input: DeriveInput) -> syn::Result<TokenStream> {
    generate_function_bodies(input.ident.span(), input.data)
        .map(|(eq_body, ne_body)| derive_with(input.ident, input.generics, eq_body, ne_body))
}

#[cfg(test)]
mod tests {
    use crate::utilities;

    #[test]
    fn test_derive_partial_eq() {
        let test_cases = [
            // Empty struct.
            (
                quote::quote! {
                    struct Foo {}
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::cmp::PartialEq for Foo {
                        fn eq(&self, other: &Self) -> bool {
                            true
                        }

                        fn ne(&self, other: &Self) -> bool {
                            false
                        }
                    }
                },
            ),
            // Struct with a single field.
            (
                quote::quote! {
                    struct Foo<T> {
                        foo: PhantomData<T>,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::cmp::PartialEq for Foo<T> {
                        fn eq(&self, other: &Self) -> bool {
                            ::core::cmp::PartialEq::eq(&self.foo, &other.foo)
                        }

                        fn ne(&self, other: &Self) -> bool {
                            ::core::cmp::PartialEq::ne(&self.foo, &other.foo)
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
                        foo: PhantomData<T>,
                        bar: PhantomData<T>,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::cmp::PartialEq for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn eq(&self, other: &Self) -> bool {
                            ::core::cmp::PartialEq::eq(&self.foo, &other.foo) &&
                            ::core::cmp::PartialEq::eq(&self.bar, &other.bar)
                        }

                        fn ne(&self, other: &Self) -> bool {
                            ::core::cmp::PartialEq::ne(&self.foo, &other.foo) ||
                            ::core::cmp::PartialEq::ne(&self.bar, &other.bar)
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
                    impl ::core::cmp::PartialEq for Foo {
                        fn eq(&self, other: &Self) -> bool {
                            true
                        }

                        fn ne(&self, other: &Self) -> bool {
                            false
                        }
                    }
                },
            ),
            // Tuple with a single field.
            (
                quote::quote! {
                    struct Foo<T>(PhantomData<T>);
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::cmp::PartialEq for Foo<T> {
                        fn eq(&self, other: &Self) -> bool {
                            ::core::cmp::PartialEq::eq(&self.0, &other.0)
                        }

                        fn ne(&self, other: &Self) -> bool {
                            ::core::cmp::PartialEq::ne(&self.0, &other.0)
                        }
                    }
                },
            ),
            // Tuple with two fields and generic constraints.
            (
                quote::quote! {
                    struct Foo<T>(PhantomData<T>, PhantomData<T>)
                    where
                        u32: Copy;
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::cmp::PartialEq for Foo<T>
                    where
                        u32: Copy
                    {
                        fn eq(&self, other: &Self) -> bool {
                            ::core::cmp::PartialEq::eq(&self.0, &other.0) &&
                            ::core::cmp::PartialEq::eq(&self.1, &other.1)
                        }

                        fn ne(&self, other: &Self) -> bool {
                            ::core::cmp::PartialEq::ne(&self.0, &other.0) ||
                            ::core::cmp::PartialEq::ne(&self.1, &other.1)
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
                    impl ::core::cmp::PartialEq for Foo {
                        fn eq(&self, other: &Self) -> bool {
                            true
                        }

                        fn ne(&self, other: &Self) -> bool {
                            false
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
                    impl ::core::cmp::PartialEq for Foo {
                        fn eq(&self, other: &Self) -> bool {
                            match *self {}
                        }

                        fn ne(&self, other: &Self) -> bool {
                            match *self {}
                        }
                    }
                },
            ),
            // Enum with a single variant.
            (
                quote::quote! {
                    enum Foo<T> {
                        Tuple1(PhantomData<T>),
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::cmp::PartialEq for Foo<T> {
                        fn eq(&self, other: &Self) -> bool {
                            match (self, other) {
                                (Self::Tuple1(self_0,), Self::Tuple1(other_0,),) =>
                                    ::core::cmp::PartialEq::eq(self_0, other_0),
                            }
                        }

                        fn ne(&self, other: &Self) -> bool {
                            match (self, other) {
                                (Self::Tuple1(self_0,), Self::Tuple1(other_0,),) =>
                                    ::core::cmp::PartialEq::ne(self_0, other_0),
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
                        Struct1 { foo: PhantomData<T> },
                        Struct2 { foo: PhantomData<T>, bar: PhantomData<T> },
                        Tuple0(),
                        Tuple1(PhantomData<T>),
                        Tuple2(PhantomData<T>, PhantomData<T>),
                        Unit,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::cmp::PartialEq for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn eq(&self, other: &Self) -> bool {
                            match (self, other) {
                                (Self::Struct0 {}, Self::Struct0 {},) => true,
                                (Self::Struct1 { foo: self_foo, }, Self::Struct1 { foo: other_foo, },) =>
                                    ::core::cmp::PartialEq::eq(self_foo, other_foo),
                                (
                                    Self::Struct2 { foo: self_foo, bar: self_bar, },
                                    Self::Struct2 { foo: other_foo, bar: other_bar, },
                                ) => ::core::cmp::PartialEq::eq(self_foo, other_foo) &&
                                    ::core::cmp::PartialEq::eq(self_bar, other_bar),
                                (Self::Tuple0(), Self::Tuple0(),) => true,
                                (Self::Tuple1(self_0,), Self::Tuple1(other_0,),) =>
                                    ::core::cmp::PartialEq::eq(self_0, other_0),
                                (Self::Tuple2(self_0, self_1,), Self::Tuple2(other_0, other_1,),) =>
                                    ::core::cmp::PartialEq::eq(self_0, other_0) &&
                                    ::core::cmp::PartialEq::eq(self_1, other_1),
                                (Self::Unit, Self::Unit,) => true,
                                _ => false,
                            }
                        }

                        fn ne(&self, other: &Self) -> bool {
                            match (self, other) {
                                (Self::Struct0 {}, Self::Struct0 {},) => false,
                                (Self::Struct1 { foo: self_foo, }, Self::Struct1 { foo: other_foo, },) =>
                                    ::core::cmp::PartialEq::ne(self_foo, other_foo),
                                (
                                    Self::Struct2 { foo: self_foo, bar: self_bar, },
                                    Self::Struct2 { foo: other_foo, bar: other_bar, },
                                ) => ::core::cmp::PartialEq::ne(self_foo, other_foo) ||
                                    ::core::cmp::PartialEq::ne(self_bar, other_bar),
                                (Self::Tuple0(), Self::Tuple0(),) => false,
                                (Self::Tuple1(self_0,), Self::Tuple1(other_0,),) =>
                                    ::core::cmp::PartialEq::ne(self_0, other_0),
                                (Self::Tuple2(self_0, self_1,), Self::Tuple2(other_0, other_1,),) =>
                                    ::core::cmp::PartialEq::ne(self_0, other_0) ||
                                    ::core::cmp::PartialEq::ne(self_1, other_1),
                                (Self::Unit, Self::Unit,) => false,
                                _ => true,
                            }
                        }
                    }
                },
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                super::derive_partial_eq(utilities::parse_derive_input(input).unwrap())
                    .unwrap()
                    .to_string(),
                expected.to_string(),
            );
        }
    }
}
