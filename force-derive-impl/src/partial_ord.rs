use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Data, DeriveInput, Fields, Generics, Ident, Index, Variant};

fn some_equal() -> TokenStream {
    quote::quote!(::core::option::Option::Some(::core::cmp::Ordering::Equal))
}

fn generate_body_helper<F>(first_field: (F, F), rest_fields: &mut impl Iterator<Item = (F, F)>) -> TokenStream
where
    F: ToTokens,
{
    let (self_field, other_field) = first_field;

    if let Some(second_field) = rest_fields.next() {
        let inner = generate_body_helper(second_field, rest_fields);

        quote::quote! {
            match ::core::cmp::PartialOrd::partial_cmp(#self_field, #other_field) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => #inner,
                cmp => cmp,
            }
        }
    } else {
        quote::quote!(::core::cmp::PartialOrd::partial_cmp(#self_field, #other_field))
    }
}

fn generate_body<F>(fields: impl IntoIterator<Item = (F, F)>) -> TokenStream
where
    F: ToTokens,
{
    let mut fields = fields.into_iter();

    if let Some(first_field) = fields.next() {
        generate_body_helper(first_field, &mut fields)
    } else {
        some_equal()
    }
}

fn generate_variant(variant: &Variant) -> TokenStream {
    let variant_name = &variant.ident;

    match &variant.fields {
        Fields::Named(fields) => {
            let self_fields = fields.named.iter().map(|field| field.ident.as_ref().unwrap());

            let self_variables = self_fields
                .clone()
                .map(|field| quote::format_ident!("self_{}", field))
                .collect::<Vec<_>>();

            let other_fields = self_fields.clone();

            let other_variables = self_fields
                .clone()
                .map(|field| quote::format_ident!("other_{}", field))
                .collect::<Vec<_>>();

            let body = generate_body(self_variables.iter().zip(&other_variables));

            quote::quote! {
                (
                    Self::#variant_name { #(#self_fields: #self_variables,)* },
                    Self::#variant_name { #(#other_fields: #other_variables,)* },
                ) => #body
            }
        }
        Fields::Unnamed(fields) => {
            let self_variables = (0..fields.unnamed.len())
                .map(|field| quote::format_ident!("self_{}", field))
                .collect::<Vec<_>>();

            let other_variables = (0..fields.unnamed.len())
                .map(|field| quote::format_ident!("other_{}", field))
                .collect::<Vec<_>>();

            let body = generate_body(self_variables.iter().zip(&other_variables));

            quote::quote! {
                (
                    Self::#variant_name(#(#self_variables,)*),
                    Self::#variant_name(#(#other_variables,)*),
                ) => #body
            }
        }
        Fields::Unit => {
            let body = some_equal();

            quote::quote! { (Self::#variant_name, Self::#variant_name,) => #body }
        }
    }
}

fn generate_function_bodies(span: Span, data: Data) -> syn::Result<TokenStream> {
    Ok(match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => generate_body(fields.named.into_iter().map(|field| {
                let field = field.ident.unwrap();

                (quote::quote!(&self.#field), quote::quote!(&other.#field))
            })),
            Fields::Unnamed(fields) => generate_body((0..fields.unnamed.len()).map(|i| {
                let field = Index::from(i);

                (quote::quote!(&self.#field), quote::quote!(&other.#field))
            })),
            Fields::Unit => some_equal(),
        },
        Data::Enum(data_enum) => {
            let variants = data_enum.variants;

            if let Some(first) = variants.first() {
                if variants.len() == 1 {
                    let arm = generate_variant(first);

                    quote::quote! {
                        match (self, other) {
                            #arm,
                        }
                    }
                } else {
                    let arms = variants.iter().map(generate_variant);

                    quote::quote! {
                        match (self, other) {
                            #(#arms,)*
                            _ => ::core::cmp::PartialOrd::partial_cmp(
                                &::core::mem::discriminant(self),
                                &::core::mem::discriminant(other),
                            ),
                        }
                    }
                }
            } else {
                quote::quote! { match *self {} }
            }
        }
        Data::Union(_) => return Err(syn::Error::new(span, "Cannot derive `PartialOrd` on a `union`.")),
    })
}

fn derive_with(ty: Ident, generics: Generics, body: TokenStream) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote::quote! {
        #[automatically_derived]
        impl #impl_generics ::core::cmp::PartialOrd for #ty #type_generics
        #where_clause
        {
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                #body
            }
        }
    }
}

pub fn derive_partial_ord(input: DeriveInput) -> syn::Result<TokenStream> {
    generate_function_bodies(input.ident.span(), input.data).map(|body| derive_with(input.ident, input.generics, body))
}

#[cfg(test)]
mod tests {
    use crate::utilities;

    #[test]
    fn test_derive_partial_ord() {
        let test_cases = [
            // Empty named struct type.
            (
                quote::quote! {
                    struct Foo {}
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::cmp::PartialOrd for Foo {
                        fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        }
                    }
                },
            ),
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
                    impl ::core::cmp::PartialOrd for Foo {
                        fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                            match ::core::cmp::PartialOrd::partial_cmp(&self.field_1, &other.field_1) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                                    ::core::cmp::PartialOrd::partial_cmp(&self.field_2, &other.field_2),
                                cmp => cmp,
                            }
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
                    impl<T, U> ::core::cmp::PartialOrd for Foo<T, U>
                    where
                        T: Trait1,
                        U: Trait2,
                    {
                        fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                            match ::core::cmp::PartialOrd::partial_cmp(&self.field_1, &other.field_1) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                                    match ::core::cmp::PartialOrd::partial_cmp(&self.field_2, &other.field_2) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                                            ::core::cmp::PartialOrd::partial_cmp(&self.field_3, &other.field_3),
                                        cmp => cmp,
                                    },
                                cmp => cmp,
                            }
                        }
                    }
                },
            ),
            // Empty tuple struct type.
            (
                quote::quote! {
                    struct Foo();
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::cmp::PartialOrd for Foo {
                        fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
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
                    impl ::core::cmp::PartialOrd for Foo {
                        fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                            match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                                    ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1),
                                cmp => cmp,
                            }
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
                    impl ::core::cmp::PartialOrd for Foo {
                        fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        }
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
                    impl ::core::cmp::PartialOrd for Foo {
                        fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                            match *self {}
                        }
                    }
                },
            ),
            // Single enum type.
            (
                quote::quote! {
                    enum Foo {
                        X(Bar)
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::cmp::PartialOrd for Foo {
                        fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                            match (self, other) {
                                (Self::X(self_0,), Self::X(other_0,),) =>
                                    ::core::cmp::PartialOrd::partial_cmp(self_0, other_0),
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
                        },
                        A(),
                        B {},
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::cmp::PartialOrd for Foo {
                        fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                            match (self, other) {
                                (Self::X, Self::X,) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                (Self::Y(self_0, self_1,), Self::Y(other_0, other_1,),) =>
                                    match ::core::cmp::PartialOrd::partial_cmp(self_0, other_0) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                                            ::core::cmp::PartialOrd::partial_cmp(self_1, other_1),
                                        cmp => cmp,
                                    },
                                (Self::Z { a: self_a, b: self_b, }, Self::Z { a: other_a, b: other_b, },) =>
                                    match ::core::cmp::PartialOrd::partial_cmp(self_a, other_a) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                                            ::core::cmp::PartialOrd::partial_cmp(self_b, other_b),
                                        cmp => cmp,
                                    },
                                (Self::A(), Self::A(),) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                (Self::B {}, Self::B {},) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                _ => ::core::cmp::PartialOrd::partial_cmp(
                                    &::core::mem::discriminant(self),
                                    &::core::mem::discriminant(other),
                                ),
                            }
                        }
                    }
                },
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                super::derive_partial_ord(utilities::parse_derive_input(input).unwrap())
                    .unwrap()
                    .to_string(),
                expected.to_string(),
            );
        }
    }
}
