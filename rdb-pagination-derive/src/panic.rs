use std::{
    fmt,
    fmt::{Display, Formatter},
};

use proc_macro2::Span;

#[derive(Debug)]
struct DisplayStringSlice<'a>(&'a [&'static str]);

impl<'a> Display for DisplayStringSlice<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for &s in self.0 {
            f.write_str("\n    ")?;
            f.write_str(s)?;
        }

        Ok(())
    }
}

#[inline]
pub(crate) fn sub_attributes_for_item(span: Span) -> syn::Error {
    syn::Error::new(
        span,
        format!(
            "available sub-attributes for the `orderByOptions` attribute:{}",
            DisplayStringSlice(&["name", "join"])
        ),
    )
}
