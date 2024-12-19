use proc_macro2::Span;
use quote::ToTokens;
use rdb_pagination_core::{Name, TableColumn, TableName};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, Ident, Lit, LitStr, Meta, MetaNameValue, Path, Token,
};

#[derive(Debug)]
pub(crate) struct Join {
    pub(crate) foreign:         TableColumn,
    pub(crate) primary:         TableColumn,
    pub(crate) real_table_name: Option<TableName>,
    pub(crate) span:            Span,
}

impl Parse for Join {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();

        let args = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

        let args_len = args.len();

        if args_len != 2 && args_len != 3 {
            return Err(syn::Error::new(input.span(), "expected 2 or 3 argumenets"));
        }

        let foreign = expr_2_two_string_tuple(&args[0])?;
        let primary = expr_2_two_string_tuple(&args[1])?;
        let real_table_name = if args_len == 3 { Some(expr_2_string(&args[2])?) } else { None };

        Ok(Self {
            foreign: (Name::Dynamic(foreign.0), Name::Dynamic(foreign.1)),
            primary: (Name::Dynamic(primary.0), Name::Dynamic(primary.1)),
            real_table_name: real_table_name.map(Name::Dynamic),
            span,
        })
    }
}

#[derive(Debug)]
pub(crate) struct OrderByOption {
    pub(crate) table_column:        TableColumn,
    pub(crate) unique:              bool,
    /// `Some(true)` means **NULL FIRST**; `Some(false)` means **NULL LAST**.
    pub(crate) nulls_first_or_last: Option<bool>,
    pub(crate) span:                Span,
}

impl Parse for OrderByOption {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();

        let args = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

        let args_len = args.len();

        if !(1..=3).contains(&args_len) {
            return Err(syn::Error::new(input.span(), "expected 1, 2 or 3 arguments"));
        }

        let table_column = expr_2_two_string_tuple(&args[0])?;

        let (unique, nulls_first_or_last) = match args_len {
            1 => (false, None),
            2 => {
                if expr_2_unique(&args[1]).is_ok() {
                    (true, None)
                } else {
                    (false, Some(expr_2_nulls_first_or_last(&args[1], false)?))
                }
            },
            3 => {
                expr_2_unique(&args[1])?;

                (true, Some(expr_2_nulls_first_or_last(&args[2], true)?))
            },
            _ => unreachable!(),
        };

        Ok(Self {
            table_column: (Name::Dynamic(table_column.0), Name::Dynamic(table_column.1)),
            unique,
            nulls_first_or_last,
            span,
        })
    }
}

#[inline]
pub(crate) fn path_to_string(path: &Path) -> String {
    path.into_token_stream().to_string().replace(' ', "")
}

#[inline]
pub(crate) fn expr_2_string(expr: &Expr) -> syn::Result<String> {
    match &expr {
        Expr::Lit(lit) => {
            if let Lit::Str(lit) = &lit.lit {
                return Ok(lit.value());
            }
        },
        Expr::Path(path) => {
            if let Some(ident) = path.path.get_ident() {
                return Ok(ident.to_string());
            }
        },
        _ => (),
    }

    Err(syn::Error::new(expr.span(), "expected an Ident"))
}

#[inline]
pub(crate) fn expr_2_two_string_tuple(expr: &Expr) -> syn::Result<(String, String)> {
    if let Expr::Tuple(tuple) = expr {
        if tuple.elems.len() != 2 {
            return Err(syn::Error::new(tuple.span(), "expected 2 elements"));
        }

        let s1 = expr_2_string(&tuple.elems[0])?;
        let s2 = expr_2_string(&tuple.elems[1])?;

        Ok((s1, s2))
    } else {
        Err(syn::Error::new(expr.span(), "expected a tuple"))
    }
}

#[inline]
pub(crate) fn expr_2_unique(expr: &Expr) -> syn::Result<()> {
    if let Expr::Path(path) = expr {
        if path.path.is_ident("unique") {
            return Ok(());
        }
    }

    Err(syn::Error::new(expr.span(), "expected `unique`"))
}

/// Return `true` if it is `nulls_first`; return `false` if it is `nulls_last`.
#[inline]
pub(crate) fn expr_2_nulls_first_or_last(expr: &Expr, after_unique: bool) -> syn::Result<bool> {
    if let Expr::Path(path) = expr {
        if let Some(ident) = path.path.get_ident() {
            match ident.to_string().as_str() {
                "nulls_first" => return Ok(true),
                "nulls_last" => return Ok(false),
                _ => (),
            }
        }
    }

    let message = if after_unique {
        "expected `nulls_first` or `nulls_last`"
    } else {
        "expected `unique`, `nulls_first` or `nulls_last`"
    };

    Err(syn::Error::new(expr.span(), message))
}

#[inline]
pub(crate) fn meta_name_value_2_string(name_value: &MetaNameValue) -> syn::Result<String> {
    match &name_value.value {
        Expr::Lit(lit) => {
            if let Lit::Str(lit) = &lit.lit {
                return Ok(lit.value());
            }
        },
        Expr::Path(path) => {
            if let Some(ident) = path.path.get_ident() {
                return Ok(ident.to_string());
            }
        },
        _ => (),
    }

    Err(syn::Error::new(
        name_value.value.span(),
        format!("expected `{path} = Ident`", path = path_to_string(&name_value.path)),
    ))
}

#[inline]
pub(crate) fn meta_2_string(meta: &Meta) -> syn::Result<String> {
    match &meta {
        Meta::NameValue(name_value) => return meta_name_value_2_string(name_value),
        Meta::List(list) => {
            if let Ok(lit) = list.parse_args::<LitStr>() {
                return Ok(lit.value());
            } else if let Ok(ident) = list.parse_args::<Ident>() {
                return Ok(ident.to_string());
            }
        },
        _ => (),
    }

    Err(syn::Error::new(
        meta.span(),
        format!("expected `{path} = Ident` or `{path}(Ident)`", path = path_to_string(meta.path())),
    ))
}
