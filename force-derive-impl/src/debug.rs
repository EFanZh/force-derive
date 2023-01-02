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

fn write_str(type_name: &str) -> TokenStream {
    quote::quote!(f.write_str(#type_name))
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
                    if fields.named.is_empty() {
                        write_str(&ty_string)
                    } else {
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
                }
                Fields::Unnamed(fields) => {
                    if fields.unnamed.is_empty() {
                        write_str(&ty_string)
                    } else {
                        let fields = (0..fields.unnamed.len()).map(Index::from);

                        quote::quote! {
                            f.debug_tuple(#ty_string)
                            #(.field(&self.#fields))*
                            .finish()
                        }
                    }
                }
                Fields::Unit => write_str(&ty_string),
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
                                if fields.named.is_empty() {
                                    let body = write_str(&variant_name_string);

                                    quote::quote! {
                                        Self::#variant_name {} => #body
                                    }
                                } else {
                                    let pattern_fields = fields.named.iter().map(|field| field.ident.as_ref().unwrap());

                                    let field_variables = pattern_fields
                                        .clone()
                                        .map(|field| quote::format_ident!("field_{}", field))
                                        .collect::<Vec<_>>();

                                    let field_names = pattern_fields.clone().map(Ident::to_string);

                                    quote::quote! {
                                        Self::#variant_name { #(#pattern_fields: #field_variables,)* } =>
                                            f.debug_struct(#variant_name_string)
                                            #(.field(#field_names, #field_variables))*
                                            .finish()
                                    }
                                }
                            }
                            Fields::Unnamed(fields) => {
                                if fields.unnamed.is_empty() {
                                    let body = write_str(&variant_name_string);

                                    quote::quote! {
                                        Self::#variant_name() => #body
                                    }
                                } else {
                                    let fields =
                                        utilities::get_field_identifiers(fields.unnamed.len()).collect::<Vec<_>>();

                                    quote::quote! {
                                        Self::#variant_name(#(#fields,)*) =>
                                            f.debug_tuple(#variant_name_string)
                                            #(.field(#fields))*
                                            .finish()
                                    }
                                }
                            }
                            Fields::Unit => {
                                let body = write_str(&variant_name_string);

                                quote::quote! {
                                    Self::#variant_name => #body
                                }
                            }
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
            // Empty struct.
            (
                quote::quote! {
                    struct Foo {}
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
            // Struct with a single field.
            (
                quote::quote! {
                    struct Foo<T> {
                        foo: PhantomData<T>,
                    }
                },
                quote::quote! {
                    #[automatically_derived]
                    impl<T> ::core::fmt::Debug for Foo<T> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            f.debug_struct("Foo").field("foo", &self.foo).finish()
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
                    impl<T> ::core::fmt::Debug for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            f.debug_struct("Foo")
                                .field("foo", &self.foo)
                                .field("bar", &self.bar)
                                .finish()
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
                    impl ::core::fmt::Debug for Foo {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            f.write_str("Foo")
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
                    impl<T> ::core::fmt::Debug for Foo<T> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            f.debug_tuple("Foo").field(&self.0).finish()
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
                    impl<T> ::core::fmt::Debug for Foo<T>
                    where
                        u32: Copy
                    {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            f.debug_tuple("Foo")
                                .field(&self.0)
                                .field(&self.1)
                                .finish()
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
                    impl ::core::fmt::Debug for Foo {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            f.write_str("Foo")
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
                    impl ::core::fmt::Debug for Foo {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
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
                    impl<T> ::core::fmt::Debug for Foo<T> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            match self {
                                Self::Tuple1(field_0,) => f.debug_tuple("Tuple1").field(field_0).finish(),
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
                    impl<T> ::core::fmt::Debug for Foo<T>
                    where
                        u32: Copy,
                    {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                            match self {
                                Self::Struct0 {} => f.write_str("Struct0"),
                                Self::Struct1 { foo: field_foo, } => f.debug_struct("Struct1")
                                    .field("foo", field_foo)
                                    .finish(),
                                Self::Struct2 { foo: field_foo, bar: field_bar, } => f.debug_struct("Struct2")
                                    .field("foo", field_foo)
                                    .field("bar", field_bar)
                                    .finish(),
                                Self::Tuple0() => f.write_str("Tuple0"),
                                Self::Tuple1(field_0,) => f.debug_tuple("Tuple1")
                                    .field(field_0)
                                    .finish(),
                                Self::Tuple2(field_0, field_1,) => f.debug_tuple("Tuple2")
                                    .field(field_0)
                                    .field(field_1)
                                    .finish(),
                                Self::Unit => f.write_str("Unit"),
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
