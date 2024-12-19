/*!
# RDB Pagination Derive

The provided crate offers a procedural macro for defining `OrderByOptions`. See the [`rdb-pagination`](https://crates.io/crates/rdb-pagination) crate.
*/

mod common;
mod panic;

use common::{meta_2_string, Join};
use proc_macro::TokenStream;
use quote::quote;
use rdb_pagination_core::{Name, OrderBuilder, Relationship};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Data, DeriveInput, Index, Meta, Token,
};

use crate::common::OrderByOption;

fn derive_input_handler(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut table_name = None;
    let mut join_list = Vec::new();

    for attr in ast.attrs.iter() {
        let path = attr.path();

        if path.is_ident("orderByOptions") {
            if let Meta::List(list) = &attr.meta {
                let result =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                for meta in result {
                    let path = meta.path();

                    if let Some(ident) = path.get_ident() {
                        match ident.to_string().as_str() {
                            "name" => {
                                if table_name.is_some() {
                                    return Err(syn::Error::new(
                                        ident.span(),
                                        "`name` has been set",
                                    ));
                                }

                                let name = meta_2_string(&meta)?;

                                table_name = Some(name);
                            },
                            "join" => {
                                if let Meta::List(list) = meta {
                                    let join: Join = list.parse_args()?;

                                    join_list.push(join);
                                } else {
                                    return Err(syn::Error::new(
                                        ident.span(),
                                        "`join` should be a list",
                                    ));
                                }
                            },
                            _ => {
                                return Err(panic::sub_attributes_for_item(path.span()));
                            },
                        }
                    } else {
                        return Err(panic::sub_attributes_for_item(path.span()));
                    }
                }
            } else {
                return Err(syn::Error::new(
                    path.span(),
                    "the `orderByOptions` attribute should be a list",
                ));
            }
        }
    }

    let mut token_stream = proc_macro2::TokenStream::new();

    if let Some(table_name) = table_name {
        let mut relationship = Relationship::new(Name::Dynamic(table_name.clone()));

        for join in join_list.iter() {
            if let Err(error) = relationship.join_check(
                join.foreign.clone(),
                join.primary.clone(),
                join.real_table_name.clone(),
            ) {
                return Err(syn::Error::new(join.span, error));
            }
        }

        if let Data::Struct(data) = ast.data {
            let mut options = Vec::with_capacity(data.fields.len());

            {
                let mut order_builder: OrderBuilder<i16> =
                    OrderBuilder::new(relationship, data.fields.len());

                for (index, field) in data.fields.iter().enumerate() {
                    let mut has_option = false;

                    for attr in field.attrs.iter() {
                        let path = attr.path();

                        if path.is_ident("orderByOptions") {
                            if has_option {
                                return Err(syn::Error::new(
                                    path.span(),
                                    "`orderByOptions` has been set",
                                ));
                            }

                            let order_by_option: OrderByOption = attr.parse_args()?;

                            if let Err(error) = order_builder.add_order_option_check(
                                order_by_option.table_column.clone(),
                                order_by_option.unique,
                            ) {
                                return Err(syn::Error::new(order_by_option.span, error));
                            }

                            has_option = true;

                            options.push((index, field, order_by_option));
                        }
                    }
                }
            }
            // Get the identifier of the type.
            let name = &ast.ident;

            let options_len = options.len();

            if options_len == 0 {
                token_stream.extend(quote! {
                    impl OrderByOptions for #name {}
                });
            } else {
                let mut join_impl = proc_macro2::TokenStream::new();

                for join in join_list {
                    let foreign_table_name = join.foreign.0.as_ref();
                    let foreign_column_name = join.foreign.1.as_ref();
                    let primary_table_name = join.primary.0.as_ref();
                    let primary_column_name = join.primary.1.as_ref();

                    let real_table_name = if let Some(real_table_name) = join.real_table_name {
                        let real_table_name = real_table_name.as_ref();

                        quote!(Some(rdb_pagination_prelude::Name::Static(#real_table_name)))
                    } else {
                        quote!(None)
                    };

                    join_impl.extend(quote! {
                        relationship.join(
                            (rdb_pagination_prelude::Name::Static(#foreign_table_name), rdb_pagination_prelude::Name::Static(#foreign_column_name)),
                            (rdb_pagination_prelude::Name::Static(#primary_table_name), rdb_pagination_prelude::Name::Static(#primary_column_name)),
                            #real_table_name
                        );
                    });
                }

                let mut options_impl = proc_macro2::TokenStream::new();

                for (index, field, option) in options {
                    let table_name = option.table_column.0.as_ref();
                    let column_name = option.table_column.1.as_ref();
                    let unique = option.unique;

                    let null_strategy =
                        if let Some(nulls_first_or_last) = option.nulls_first_or_last {
                            if nulls_first_or_last {
                                quote!(rdb_pagination_prelude::NullStrategy::First)
                            } else {
                                quote!(rdb_pagination_prelude::NullStrategy::Last)
                            }
                        } else {
                            quote!(rdb_pagination_prelude::NullStrategy::Default)
                        };

                    let order_method = if let Some(ident) = &field.ident {
                        quote!(self.#ident)
                    } else {
                        let index = Index::from(index);

                        quote!(self.#index)
                    };

                    options_impl.extend(quote! {
                        order_builder.add_order_option(
                            (rdb_pagination_prelude::Name::Static(#table_name), rdb_pagination_prelude::Name::Static(#column_name)),
                            #unique,
                            #null_strategy,
                            #order_method,
                        );
                    });
                }

                let order_by_options_impl = quote! {
                    impl OrderByOptions for #name {
                        fn to_sql(&self) -> (::std::vec::Vec<rdb_pagination_prelude::SqlJoin>, ::std::vec::Vec<rdb_pagination_prelude::SqlOrderByComponent>) {
                            let mut relationship = rdb_pagination_prelude::Relationship::new(rdb_pagination_prelude::Name::Static(#table_name));

                            #join_impl

                            let mut order_builder = rdb_pagination_prelude::OrderBuilder::new(relationship, #options_len);

                            #options_impl

                            order_builder.build()
                        }
                    }
                };

                token_stream.extend(order_by_options_impl);
            }
        } else {
            return Err(syn::Error::new(
                ast.ident.span(),
                "should use a struct to implement `OrderByOptions`",
            ));
        }
    }

    Ok(token_stream)
}

#[proc_macro_derive(OrderByOptions, attributes(orderByOptions))]
pub fn order_by_options_derive(input: TokenStream) -> TokenStream {
    struct MyDeriveInput(proc_macro2::TokenStream);

    impl Parse for MyDeriveInput {
        #[inline]
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let token_stream = derive_input_handler(input.parse::<DeriveInput>()?)?;

            Ok(Self(token_stream))
        }
    }

    // Parse the token stream
    let derive_input = parse_macro_input!(input as MyDeriveInput);

    derive_input.0.into()
}
