use crate::OrderByOptions;

/// Struct representing pagination options.
///
/// # Examples
///
/// ```rust
/// # use rdb_pagination_core::PaginationOptions;
/// #
/// let options = PaginationOptions::new().page(1).items_per_page(20);
/// ```
#[derive(Debug, Clone)]
pub struct PaginationOptions<T: OrderByOptions = ()> {
    /// Page number.
    ///
    /// * If the value is `0`, it will default to `1`.
    /// * If the value exceeds the maximum page number, it will be considered as the maximum page number.
    ///
    ///  Default: `1`.
    pub page:           usize,
    /// Number of items per page.
    ///
    /// * If the value is `0`, it means **query all items in a single page**.
    ///
    ///  Default: `0`.
    pub items_per_page: usize,
    /// Ordering options which has to implement the `OrderByOptions` trait.
    ///
    /// Default: `()`.
    pub order_by:       T,
}

impl PaginationOptions {
    /// Create a new `PaginationOptions`.
    ///
    /// ```rust
    /// # use rdb_pagination_core::PaginationOptions;
    /// #
    /// let options = PaginationOptions::new();
    /// // equals to
    /// let options = PaginationOptions {
    ///     page:           1,
    ///     items_per_page: 0,
    ///     order_by:       (),
    /// };
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self {
            page: 1, items_per_page: 0, order_by: ()
        }
    }
}

impl<T: OrderByOptions> Default for PaginationOptions<T> {
    /// Create a new `PaginationOptions<T>`.
    ///
    /// ```rust
    /// # use rdb_pagination_core::PaginationOptions;
    /// #
    /// let options = <PaginationOptions<()>>::default();
    /// // equals to
    /// let options = PaginationOptions {
    ///     page:           1,
    ///     items_per_page: 0,
    ///     order_by:       (),
    /// };
    /// ```
    #[inline]
    fn default() -> Self {
        Self {
            page: 1, items_per_page: 0, order_by: T::default()
        }
    }
}

impl<T: OrderByOptions> PaginationOptions<T> {
    /// Set the page number.
    ///
    /// * If the value is `0`, it will be considered as `1`.
    /// * If the value exceeds the maximum page number, it will be considered as the maximum page number.
    #[inline]
    pub const fn page(mut self, page: usize) -> Self {
        self.page = page;

        self
    }

    /// Set the number of items per page.
    ///
    /// * If the value is `0`, it means **query all items in a single page**.
    #[inline]
    pub const fn items_per_page(mut self, items_per_page: usize) -> Self {
        self.items_per_page = items_per_page;

        self
    }

    /// Set the ordering options which has to implement the `OrderByOptions` trait.
    #[inline]
    pub fn order_by(mut self, order_by: T) -> Self {
        self.order_by = order_by;

        self
    }

    /// Compute the offset for pagination.
    #[inline]
    pub const fn offset(&self) -> u64 {
        if self.items_per_page == 0 {
            0
        } else {
            match self.page {
                0 | 1 => 0,
                _ => (self.items_per_page * (self.page - 1)) as u64,
            }
        }
    }

    /// Compute the limit for pagination. `None` means **unlimited**.
    #[inline]
    pub const fn limit(&self) -> Option<usize> {
        if self.items_per_page == 0 {
            None
        } else {
            Some(self.items_per_page)
        }
    }
}

#[cfg(feature = "mysql")]
impl<T: OrderByOptions> PaginationOptions<T> {
    /// Generate a `LIMIT with OFFSET` clause for MySQL.
    ///
    /// If `limit()` is `Some(n)`,
    ///
    /// ```sql
    /// LIMIT <limit()> [OFFSET <offset()>]
    /// ```
    ///
    /// If `offset()` is not zero,
    ///
    /// ```sql
    /// [LIMIT <limit()>] OFFSET <offset()>
    /// ```
    pub fn to_mysql_limit_offset<'a>(&self, s: &'a mut String) -> &'a str {
        use std::{fmt::Write, str::from_utf8_unchecked};

        let len = s.len();

        let limit = self.limit();

        if let Some(limit) = limit {
            s.write_fmt(format_args!("LIMIT {limit}")).unwrap();
        }

        let offset = self.offset();

        if offset > 0 {
            if !s.is_empty() {
                s.push(' ');
            }

            s.write_fmt(format_args!("OFFSET {offset}")).unwrap();
        }

        unsafe { from_utf8_unchecked(&s.as_bytes()[len..]) }
    }
}

#[cfg(feature = "serde")]
mod serde_trait {
    use core::{fmt, fmt::Formatter, marker::PhantomData};

    use serde::{
        de::{MapAccess, Visitor},
        ser::SerializeStruct,
        Deserialize, Deserializer, Serialize, Serializer,
    };

    use super::PaginationOptions;
    use crate::OrderByOptions;

    impl<'de, T: OrderByOptions + Deserialize<'de>> Deserialize<'de> for PaginationOptions<T> {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>, {
            struct MyVisitor<T>(PhantomData<T>);

            impl<'de, T: OrderByOptions + Deserialize<'de>> Visitor<'de> for MyVisitor<T> {
                type Value = PaginationOptions<T>;

                #[inline]
                fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                    f.write_str("a map of options")
                }

                #[inline]
                fn visit_map<V>(self, mut v: V) -> Result<Self::Value, V::Error>
                where
                    V: MapAccess<'de>, {
                    let mut page: Option<usize> = None;
                    let mut items_per_page: Option<usize> = None;
                    let mut order_by: Option<T> = None;

                    while let Some(key) = v.next_key::<&str>()? {
                        match key {
                            "page" => {
                                page = Some(v.next_value()?);
                            },
                            "items_per_page" => {
                                items_per_page = Some(v.next_value()?);
                            },
                            "order_by" => {
                                order_by = Some(v.next_value()?);
                            },
                            _ => continue,
                        }
                    }

                    let page = page.unwrap_or(1);
                    let items_per_page = items_per_page.unwrap_or(0);
                    let order_by = order_by.unwrap_or_default();

                    Ok(PaginationOptions {
                        page,
                        items_per_page,
                        order_by,
                    })
                }
            }

            deserializer.deserialize_map(MyVisitor(PhantomData))
        }
    }

    impl<T: OrderByOptions + Serialize> Serialize for PaginationOptions<T> {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer, {
            let mut s = serializer.serialize_struct("PaginationOptions", 3)?;

            s.serialize_field("page", &self.page)?;
            s.serialize_field("items_per_page", &self.items_per_page)?;
            s.serialize_field("order_by", &self.order_by)?;

            s.end()
        }
    }
}
