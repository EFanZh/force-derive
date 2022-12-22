use proc_macro2::{Literal, TokenStream};
use syn::{Data, DeriveInput, Fields, Generics, Ident};

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
                    let fields = (0..fields.unnamed.len()).map(Literal::usize_unsuffixed);

                    quote::quote! { Self(#(#clone(&self.#fields),)*) }
                }
                Fields::Unit => quote::quote! { Self },
            },
            Data::Enum(data_enum) => {
                let variants = data_enum.variants.into_iter().map(|variant| {
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
                            let fields = (0..fields.unnamed.len())
                                .map(|i| quote::format_ident!("field_{}", i))
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
                        #(#variants,)*
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
                    impl ::core::clone::Clone for Foo {
                        fn clone(&self) -> Self {
                            Self { field_1: ::core::clone::Clone::clone(&self.field_1), field_2: ::core::clone::Clone::clone(&self.field_2), }
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
                    impl<T, U> ::core::clone::Clone for Foo<T, U>
                    where
                        T: Trait1,
                        U: Trait2,
                    {
                        fn clone(&self) -> Self {
                            Self {
                                field_1: ::core::clone::Clone::clone(&self.field_1),
                                field_2: ::core::clone::Clone::clone(&self.field_2),
                                field_3: ::core::clone::Clone::clone(&self.field_3),
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
                    impl ::core::clone::Clone for Foo {
                        fn clone(&self) -> Self {
                            Self(::core::clone::Clone::clone(&self.0), ::core::clone::Clone::clone(&self.1),)
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
                    impl ::core::clone::Clone for Foo {
                        fn clone(&self) -> Self {
                            Self
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
                    impl ::core::clone::Clone for Foo {
                        fn clone(&self) -> Self {
                            match self {
                                Self::X => Self::X,
                                Self::Y(field_0, field_1,) => Self::Y(::core::clone::Clone::clone(field_0), ::core::clone::Clone::clone(field_1),),
                                Self::Z { a, b, } => Self::Z { a: ::core::clone::Clone::clone(a), b: ::core::clone::Clone::clone(b), },
                            }
                        }
                    }
                },
            ),
            // Union type.
            (
                quote::quote! {
                    union Foo {
                        x: i32,
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
