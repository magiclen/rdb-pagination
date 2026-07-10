/// Struct representing pagination information.
///
/// # Examples
///
/// ```rust
/// # use rdb_pagination_core::Pagination;
/// #
/// let pagination =
///     Pagination::new().items_per_page(20).total_items(50).page(1);
///
/// let total_pages = pagination.get_total_pages(); // 3
/// ```
///
/// With the `serde` feature, deserialization rejects values whose page or total page count is inconsistent.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "PaginationInput"))]
pub struct Pagination {
    page:           usize,
    total_pages:    usize,
    items_per_page: usize,
    total_items:    usize,
}

impl Pagination {
    #[allow(clippy::new_without_default)]
    /// Create a new `Pagination`.
    ///
    /// ```rust
    /// # use rdb_pagination_core::Pagination;
    /// #
    /// let pagination = Pagination::new();
    /// // equals to
    /// // let pagination = Pagination {
    /// //     page:           1,
    /// //     total_pages:    1,
    /// //     items_per_page: 0,
    /// //     total_items:    0,
    /// // };
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self {
            page: 1, total_pages: 1, items_per_page: 0, total_items: 0
        }
    }

    /// Set the page number.
    ///
    /// * If the value is `0`, it will be changed to `1`.
    /// * If `total_pages` is `0`, the value will be changed to `1`.
    /// * If the value is bigger than `total_pages`, it will be changed to `total_pages`.
    #[inline]
    pub const fn page(mut self, page: usize) -> Self {
        self.page = Self::normalize_page(page, self.total_pages);

        self
    }

    /// Number of items per page.
    ///
    /// * If the value is `0`, it means **all items in a single page**.
    #[inline]
    pub const fn items_per_page(mut self, items_per_page: usize) -> Self {
        self.items_per_page = items_per_page;

        self.update_total_pages()
    }

    /// Total number of items.
    #[inline]
    pub const fn total_items(mut self, total_items: usize) -> Self {
        self.total_items = total_items;

        self.update_total_pages()
    }

    #[inline]
    const fn update_total_pages(mut self) -> Self {
        self.total_pages = Self::calculate_total_pages(self.items_per_page, self.total_items);
        self.page = Self::normalize_page(self.page, self.total_pages);

        self
    }

    #[inline]
    const fn calculate_total_pages(items_per_page: usize, total_items: usize) -> usize {
        match items_per_page {
            0 => 1,
            1 => total_items,
            _ => total_items.div_ceil(items_per_page),
        }
    }

    #[inline]
    const fn normalize_page(page: usize, total_pages: usize) -> usize {
        if page == 0 || total_pages == 0 {
            1
        } else if page > total_pages {
            total_pages
        } else {
            page
        }
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct PaginationInput {
    page:           usize,
    total_pages:    usize,
    items_per_page: usize,
    total_items:    usize,
}

#[cfg(feature = "serde")]
impl TryFrom<PaginationInput> for Pagination {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PaginationInput) -> Result<Self, Self::Error> {
        let total_pages = Self::calculate_total_pages(value.items_per_page, value.total_items);

        if value.total_pages != total_pages {
            return Err("total_pages does not match items_per_page and total_items");
        }

        if value.page != Self::normalize_page(value.page, value.total_pages) {
            return Err("page is outside the valid range");
        }

        Ok(Self {
            page:           value.page,
            total_pages:    value.total_pages,
            items_per_page: value.items_per_page,
            total_items:    value.total_items,
        })
    }
}

impl Pagination {
    /// Get the page number.
    #[inline]
    pub const fn get_page(&self) -> usize {
        self.page
    }

    /// Get the total number of pages.
    #[inline]
    pub const fn get_total_pages(&self) -> usize {
        self.total_pages
    }

    /// Number of items per page.
    ///
    /// * If the value is `0`, it means **all items in a single page**.
    #[inline]
    pub const fn get_items_per_page(&self) -> usize {
        self.items_per_page
    }

    /// Get total number of items.
    #[inline]
    pub const fn get_total_items(&self) -> usize {
        self.total_items
    }
}
