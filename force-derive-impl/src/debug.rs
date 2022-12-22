use proc_macro2::{Literal, TokenStream};
use syn::{Data, DeriveInput, Fields, Generics, Ident};

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

pub fn derive_debug(input: DeriveInput) -> TokenStream {
    let ty_string = input.ident.to_string();

    derive_with(
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
                    let fields = (0..fields.unnamed.len()).map(Literal::usize_unsuffixed);

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
                let variants = data_enum.variants.into_iter().map(|variant| {
                    let variant_name = variant.ident;
                    let variant_name_string = variant_name.to_string();

                    match variant.fields {
                        Fields::Named(fields) => {
                            let pattern_fields = fields
                                .named
                                .iter()
                                .map(|field| field.ident.as_ref().unwrap());

                            let field_names = pattern_fields.clone().map(Ident::to_string);

                            let expression_fields = pattern_fields.clone();

                            quote::quote! {
                                Self::#variant_name { #(#pattern_fields,)* } =>
                                    f.debug_struct(#variant_name_string)
                                    #(.field(#field_names, #expression_fields))*
                                    .finish()
                            }
                        }
                        Fields::Unnamed(fields) => {
                            let fields = (0..fields.unnamed.len())
                                .map(|i| quote::format_ident!("field_{}", i))
                                .collect::<Vec<_>>();

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
                        #(#variants,)*
                    }
                }
            }
            Data::Union(_) => panic!("Cannot derive `Debug` on a `union`."),
        },
    )
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
                                Self::Z { a, b, } => f.debug_struct("Z").field("a", a).field("b", b).finish(),
                            }
                        }
                    }
                },
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                super::derive_debug(utilities::parse_derive_input(input).unwrap()).to_string(),
                expected.to_string()
            );
        }
    }
}