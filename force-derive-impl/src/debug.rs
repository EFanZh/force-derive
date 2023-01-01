use crate::utilities;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Fields, Generics, Ident, Index};

fn derive_with(ty: Ident, generics: Generics, body: TokenStream) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote::quote! {
        #[automatically_derived]
        impl #impl_generics ::core::fmt::Debug for #ty #type_generics
        #where_clause
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                #body
            }
        }
    }
}

pub fn derive_debug(input: DeriveInput) -> syn::Result<TokenStream> {
    let span = input.ident.span();
    let ty_string = input.ident.to_string();

    Ok(derive_with(
        input.ident,
        input.generics,
        match input.data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields) => {
                    let fields = fields.named.into_iter().map(|field| {
                        let field = field.ident.unwrap();
                        let field_str = field.to_string();

                        quote::quote!(#field_str, &self.#field)
                    });

                    quote::quote! {
                        f.debug_struct(#ty_string)
                        #(.field(#fields))*
                        .finish()
                    }
                }
                Fields::Unnamed(fields) => {
                    let fields = (0..fields.unnamed.len()).map(Index::from);

                    quote::quote! {
                        f.debug_tuple(#ty_string)
                        #(.field(&self.#fields))*
                        .finish()
                    }
                }
                Fields::Unit => quote::quote! {
                    f.write_str(#ty_string)
                },
            },
            Data::Enum(data_enum) => {
                let variants = data_enum.variants;

                if variants.is_empty() {
                    quote::quote! { match *self {} }
                } else {
                    let arms = variants.into_iter().map(|variant| {
                        let variant_name = variant.ident;
                        let variant_name_string = variant_name.to_string();

                        match variant.fields {
                            Fields::Named(fields) => {
                                let pattern_fields = fields.named.iter().map(|field| field.ident.as_ref().unwrap());

                                let field_variables =
                                    utilities::get_field_identifiers(fields.named.len()).collect::<Vec<_>>();

                                let field_names = pattern_fields.clone().map(Ident::to_string);

                                quote::quote! {
                                    Self::#variant_name { #(#pattern_fields: #field_variables,)* } =>
                                        f.debug_struct(#variant_name_string)
                                        #(.field(#field_names, #field_variables))*
                                        .finish()
                                }
                            }
                            Fields::Unnamed(fields) => {
                                let fields = utilities::get_field_identifiers(fields.unnamed.len()).collect::<Vec<_>>();

                                quote::quote! {
                                    Self::#variant_name(#(#fields,)*) =>
                                        f.debug_tuple(#variant_name_string)
                                        #(.field(#fields))*
                                        .finish()
                                }
                            }
                            Fields::Unit => quote::quote! {
                                Self::#variant_name => f.write_str(#variant_name_string)
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
            Data::Union(_) => return Err(syn::Error::new(span, "Cannot derive `Debug` on a `union`.")),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::utilities;

    #[test]
    fn test_derive_debug() {
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
                    impl ::core::fmt::Debug for Foo {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            f.debug_struct("Foo")
                                .field("field_1", &self.field_1)
                                .field("field_2", &self.field_2)
                                .finish()
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
                    impl<T, U> ::core::fmt::Debug for Foo<T, U>
                    where
                        T: Trait1,
                        U: Trait2,
                    {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            f.debug_struct("Foo")
                                .field("field_1", &self.field_1)
                                .field("field_2", &self.field_2)
                                .field("field_3", &self.field_3)
                                .finish()
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
                    impl ::core::fmt::Debug for Foo {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            f.debug_tuple("Foo")
                                .field(&self.0)
                                .field(&self.1)
                                .finish()
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
                    impl ::core::fmt::Debug for Foo {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            f.write_str("Foo")
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
                    impl ::core::fmt::Debug for Foo {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
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
                    impl ::core::fmt::Debug for Foo {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            match self {
                                Self::X => f.write_str("X"),
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
                    impl ::core::fmt::Debug for Foo {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            match self {
                                Self::X => f.write_str("X"),
                                Self::Y(field_0, field_1,) => f.debug_tuple("Y").field(field_0).field(field_1).finish(),
                                Self::Z { a: field_0, b: field_1, } => f.debug_struct("Z").field("a", field_0).field("b", field_1).finish(),
                            }
                        }
                    }
                },
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                super::derive_debug(utilities::parse_derive_input(input).unwrap())
                    .unwrap()
                    .to_string(),
                expected.to_string(),
            );
        }
    }
}
