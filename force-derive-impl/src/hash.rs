use crate::utilities;
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
            let field_names = fields.named.iter().map(|field| field.ident.as_ref().unwrap());

            let field_variables = field_names
                .clone()
                .map(|field| quote::format_ident!("field_{}", field))
                .collect::<Vec<_>>();

            quote::quote! {
                Self::#variant_name { #(#field_names: #field_variables,)* } => {
                    #(#hash(#field_variables, state);)*
                }
            }
        }
        Fields::Unnamed(fields) => {
            let fields = utilities::get_field_identifiers(fields.unnamed.len()).collect::<Vec<_>>();

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
            Data::Union(_) => return Err(syn::Error::new(span, "Cannot derive `Hash` on a `union`.")),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::utilities;

    #[test]
    fn test_derive_hash() {
        let test_cases = [
            // Empty struct.
            (
                quote::quote! {
                    struct Foo {}
                },
                quote::quote! {
                    #[automatically_derived]
                    impl ::core::hash::Hash for Foo {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {}
                    }
                },
            ),
            // Struct with a single field.
            (
                quote::quote! {
                    struct Foo<T> {
                        foo: ForceHash<T>,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::hash::Hash for Foo<T> {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            ::core::hash::Hash::hash(&self.foo, state);
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
                        foo: ForceHash<T>,
                        bar: ForceHash<T>,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::hash::Hash for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            ::core::hash::Hash::hash(&self.foo, state);
                            ::core::hash::Hash::hash(&self.bar, state);
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
                    impl ::core::hash::Hash for Foo {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {}
                    }
                },
            ),
            // Tuple with a single field.
            (
                quote::quote! {
                    struct Foo<T>(ForceHash<T>);
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::hash::Hash for Foo<T> {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            ::core::hash::Hash::hash(&self.0, state);
                        }
                    }
                },
            ),
            // Tuple with two fields and generic constraints.
            (
                quote::quote! {
                    struct Foo<T>(ForceHash<T>, ForceHash<T>)
                    where
                        u32: Copy;
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::hash::Hash for Foo<T>
                    where
                        u32: Copy
                    {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            ::core::hash::Hash::hash(&self.0, state);
                            ::core::hash::Hash::hash(&self.1, state);
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
                    impl ::core::hash::Hash for Foo {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {}
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
                    impl ::core::hash::Hash for Foo {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            match *self {}
                        }
                    }
                },
            ),
            // Enum with a single variant.
            (
                quote::quote! {
                    enum Foo<T> {
                        Tuple1(ForceHash<T>),
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::hash::Hash for Foo<T> {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            match self {
                                Self::Tuple1(field_0,) => {
                                    ::core::hash::Hash::hash(field_0, state);
                                },
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
                        Struct1 { foo: ForceHash<T> },
                        Struct2 { foo: ForceHash<T>, bar: ForceHash<T> },
                        Tuple0(),
                        Tuple1(ForceHash<T>),
                        Tuple2(ForceHash<T>, ForceHash<T>),
                        Unit,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::hash::Hash for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                            ::core::hash::Hash::hash(&::core::mem::discriminant(self), state);

                            match self {
                                Self::Struct0 {} => {},
                                Self::Struct1 { foo: field_foo, } => {
                                    ::core::hash::Hash::hash(field_foo, state);
                                },
                                Self::Struct2 { foo: field_foo, bar: field_bar, } => {
                                    ::core::hash::Hash::hash(field_foo, state);
                                    ::core::hash::Hash::hash(field_bar, state);
                                },
                                Self::Tuple0() => {},
                                Self::Tuple1(field_0,) => {
                                    ::core::hash::Hash::hash(field_0, state);
                                },
                                Self::Tuple2(field_0, field_1,) => {
                                    ::core::hash::Hash::hash(field_0, state);
                                    ::core::hash::Hash::hash(field_1, state);
                                },
                                Self::Unit => {},
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
