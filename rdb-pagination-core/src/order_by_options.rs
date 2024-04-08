use crate::{SqlJoin, SqlOrderByComponent};

/// Options for the `ORDER BY` clause.
pub trait OrderByOptions: Default {
    /// Create objects for generating a SQL statement.
    #[inline]
    fn to_sql(&self) -> (Vec<SqlJoin>, Vec<SqlOrderByComponent>) {
        (Vec::new(), Vec::new())
    }
}

impl OrderByOptions for () {}
