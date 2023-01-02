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
            // Empty struct.
            (
                quote::quote! {
                    struct Foo {}
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::marker::Copy for Foo {}
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
                    impl<T> ::core::marker::Copy for Foo<T> {}
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
                    impl<T> ::core::marker::Copy for Foo<T>
                    where
                        u32: Copy,
                    {
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
                    impl ::core::marker::Copy for Foo {}
                },
            ),
            // Tuple with a single field.
            (
                quote::quote! {
                    struct Foo<T>(PhantomData<T>);
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::marker::Copy for Foo<T> {}
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
                    impl<T> ::core::marker::Copy for Foo<T>
                    where
                        u32: Copy
                    {
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
                    impl ::core::marker::Copy for Foo {}
                },
            ),
            // Empty enum.
            (
                quote::quote! {
                    enum Foo {}
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::marker::Copy for Foo {}
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
                    impl<T> ::core::marker::Copy for Foo<T> {}
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
                    impl<T> ::core::marker::Copy for Foo<T>
                    where
                        u32: Copy,
                    {
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
