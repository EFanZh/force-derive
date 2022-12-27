use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn derive_copy(input: DeriveInput) -> TokenStream {
    crate::marker_trait::derive_marker_trait(input, quote::quote!(::core::marker::Copy))
}

#[cfg(test)]
mod tests {
    use crate::utilities;

    #[test]
    fn test_derive_copy() {
        let test_cases = [
            // Struct type.
            (
                quote::quote! {
                    struct Foo {
                        field_1: Type1,
                        field_2: Type2
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::marker::Copy for Foo {}
                },
            ),
            // Generic struct type.
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
                    impl<T, U> ::core::marker::Copy for Foo<T, U>
                    where
                        T: Trait1,
                        U: Trait2,
                    {
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
                    impl ::core::marker::Copy for Foo {}
                },
            ),
            // Empty enum type.
            (
                quote::quote! {
                    enum Foo {}
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::marker::Copy for Foo {}
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
                    impl ::core::marker::Copy for Foo {}
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
                    impl ::core::marker::Copy for Foo {}
                },
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                super::derive_copy(utilities::parse_derive_input(input).unwrap()).to_string(),
                expected.to_string(),
            );
        }
    }
}
