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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Pagination {
    page:           usize,
    total_pages:    usize,
    items_per_page: usize,
    total_items:    usize,
}

impl Pagination {
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
    /// * If the value is bigger than `total_pages`, it will be changed to `total_pages`.
    #[inline]
    pub const fn page(mut self, page: usize) -> Self {
        if page == 0 {
            self.page = 1;
        } else if page > self.total_pages {
            self.page = self.total_pages;
        } else {
            self.page = page;
        }

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
        match self.items_per_page {
            0 => {
                self.total_pages = 1;
            },
            1 => self.total_pages = self.total_items,
            _ => {
                self.total_pages =
                    (self.total_items + (self.items_per_page - 1)) / self.items_per_page
            },
        }

        if self.page < self.total_pages {
            self.page = self.total_pages;
        }

        self
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
