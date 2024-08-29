use crate::{ColumnName, OrderType, TableName};

/// Struct for generating the `ORDER BY` clause.
#[derive(Debug, Clone)]
pub struct SqlOrderByComponent {
    pub table_name:  TableName,
    pub column_name: ColumnName,
    pub order_type:  OrderType,
}

#[cfg(any(feature = "mysql", feature = "sqlite"))]
impl SqlOrderByComponent {
    fn to_sql_order_by_clause_component<'a>(&self, s: &'a mut String) -> &'a str {
        use std::{fmt::Write, str::from_utf8_unchecked};

        let len = s.len();

        s.write_fmt(format_args!(
            "`{table_name}`.`{column_name}` {order_type}",
            table_name = self.table_name,
            column_name = self.column_name,
            order_type = self.order_type.as_str(),
        ))
        .unwrap();

        unsafe { from_utf8_unchecked(&s.as_bytes()[len..]) }
    }

    fn format_sql_order_by_components<'a>(
        order_by_components: &[SqlOrderByComponent],
        s: &'a mut String,
    ) -> &'a str {
        use std::str::from_utf8_unchecked;

        if order_by_components.is_empty() {
            return "";
        }

        let len = s.len();

        s.push_str("ORDER BY ");

        for order_by_unit in order_by_components {
            order_by_unit.to_sql_order_by_clause_component(s);
            s.push_str(", ");
        }

        unsafe {
            let len = s.len();

            s.as_mut_vec().truncate(len - 2);
        }

        unsafe { from_utf8_unchecked(&s.as_bytes()[len..]) }
    }
}

#[cfg(feature = "mysql")]
impl SqlOrderByComponent {
    /// Generate an `ORDER BY` component for MySQL.
    ///
    /// ```sql
    /// `<table_name>`.`<column_name>` <order_type>
    /// ```
    #[inline]
    pub fn to_mysql_order_by_clause_component<'a>(&self, s: &'a mut String) -> &'a str {
        self.to_sql_order_by_clause_component(s)
    }

    /// Generate an `ORDER BY` clause for MySQL.
    ///
    /// If there is at least one component, the result string will starts with `ORDER BY`, and concatenate a series of `SqlOrderByComponent`s with `,`.
    ///
    /// ```sql
    /// ORDER BY <SqlOrderByComponent[0]>, <SqlOrderByComponent[1]>
    /// ```
    #[inline]
    pub fn format_mysql_order_by_components<'a>(
        order_by_components: &[SqlOrderByComponent],
        s: &'a mut String,
    ) -> &'a str {
        Self::format_sql_order_by_components(order_by_components, s)
    }
}

#[cfg(feature = "sqlite")]
impl SqlOrderByComponent {
    /// Generate an `ORDER BY` component for SQLite.
    ///
    /// ```sql
    /// `<table_name>`.`<column_name>` <order_type>
    /// ```
    #[inline]
    pub fn to_sqlite_order_by_clause_component<'a>(&self, s: &'a mut String) -> &'a str {
        self.to_sql_order_by_clause_component(s)
    }

    /// Generate an `ORDER BY` clause for SQLite.
    ///
    /// If there is at least one component, the result string will starts with `ORDER BY`, and concatenate a series of `SqlOrderByComponent`s with `,`.
    ///
    /// ```sql
    /// ORDER BY <SqlOrderByComponent[0]>, <SqlOrderByComponent[1]>
    /// ```
    #[inline]
    pub fn format_sqlite_order_by_components<'a>(
        order_by_components: &[SqlOrderByComponent],
        s: &'a mut String,
    ) -> &'a str {
        Self::format_sql_order_by_components(order_by_components, s)
    }
}
