use std::{
    error::Error,
    fmt,
    fmt::{Display, Formatter},
};

use crate::{ColumnName, TableColumnAttributes, TableName};

/// Struct for generating the `JOIN` clause.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SqlJoin {
    pub other_table_name:  TableName,
    pub other_column_name: ColumnName,
    pub real_table_name:   Option<TableName>,
    pub using_table_name:  TableName,
    pub using_column_name: ColumnName,
}

impl SqlJoin {
    #[doc(hidden)]
    #[inline]
    pub fn from_table_column_attributes(table_column_attributes: &TableColumnAttributes) -> Self {
        Self {
            other_table_name:  table_column_attributes.table_name.clone(),
            other_column_name: table_column_attributes.column_name.clone(),
            real_table_name:   table_column_attributes.real_table_name.clone(),
            using_table_name:  table_column_attributes.foreign_table_name.clone(),
            using_column_name: table_column_attributes.foreign_column_name.clone(),
        }
    }
}

#[cfg(feature = "mysql")]
impl SqlJoin {
    /// Generate a `JOIN` clause for MySQL.
    ///
    /// If `real_table_name` exists,
    ///
    /// ```sql
    /// JOIN `<real_table_name>` AS `<other_table_name>` ON `<other_table_name>`.`<other_column_name>` = `<using_table_name>`.`<using_column_name>`
    /// ```
    ///
    /// or
    ///
    /// ```sql
    /// JOIN `<other_table_name>` ON `<other_table_name>`.`<other_column_name>` = `<using_table_name>`.`<using_column_name>`
    /// ```
    pub fn to_mysql_join_clause<'a>(&self, s: &'a mut String) -> &'a str {
        use std::{fmt::Write, str::from_utf8_unchecked};

        let len = s.len();

        if let Some(real_table_name) = &self.real_table_name {
            s.write_fmt(format_args!(
                "LEFT JOIN `{real_table_name}` AS `{other_table_name}` ON \
                 `{other_table_name}`.`{other_column_name}` = \
                 `{using_table_name}`.`{using_column_name}`",
                other_table_name = self.other_table_name,
                other_column_name = self.other_column_name,
                using_table_name = self.using_table_name,
                using_column_name = self.using_column_name,
            ))
            .unwrap()
        } else {
            s.write_fmt(format_args!(
                "LEFT JOIN `{other_table_name}` ON `{other_table_name}`.`{other_column_name}` = \
                 `{using_table_name}`.`{using_column_name}`",
                other_table_name = self.other_table_name,
                other_column_name = self.other_column_name,
                using_table_name = self.using_table_name,
                using_column_name = self.using_column_name,
            ))
            .unwrap()
        }

        unsafe { from_utf8_unchecked(&s.as_bytes()[len..]) }
    }

    /// Generate `JOIN` clauses for MySQL.
    ///
    /// Concatenate a series of `SqlJoin`s with `\n`.
    pub fn format_mysql_join_clauses<'a>(joins: &[SqlJoin], s: &'a mut String) -> &'a str {
        use std::str::from_utf8_unchecked;

        if joins.is_empty() {
            return "";
        }

        let len = s.len();

        for join in joins {
            join.to_mysql_join_clause(s);
            s.push('\n');
        }

        unsafe {
            let len = s.len();

            s.as_mut_vec().truncate(len - 1);
        }

        unsafe { from_utf8_unchecked(&s.as_bytes()[len..]) }
    }
}

/// Operators for `SqlJoin`s.
pub trait SqlJoinsOps {
    /// Insert a `SqlJoin` if it does not exist. Return `Ok(true)` if a new `SqlJoin` has been pushed.
    fn add_join(&mut self, join: SqlJoin) -> Result<bool, SqlJoinsInsertError>;
}

impl SqlJoinsOps for Vec<SqlJoin> {
    #[inline]
    fn add_join(&mut self, join: SqlJoin) -> Result<bool, SqlJoinsInsertError> {
        if let Some(existing_join) = self
            .iter()
            .find(|existing_join| existing_join.other_table_name == join.other_table_name)
        {
            if existing_join.other_column_name != join.other_column_name
                || existing_join.real_table_name != join.real_table_name
                || existing_join.using_table_name != join.using_table_name
                || existing_join.using_column_name != join.using_column_name
            {
                Err(SqlJoinsInsertError::OtherTableNameConflict)
            } else {
                Ok(false)
            }
        } else {
            self.push(join);

            Ok(true)
        }
    }
}

#[derive(Debug, Clone)]
pub enum SqlJoinsInsertError {
    OtherTableNameConflict,
}

impl Display for SqlJoinsInsertError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::OtherTableNameConflict => {
                f.write_str("other_table_name exists but the join clauses are not exactly the same")
            },
        }
    }
}

impl Error for SqlJoinsInsertError {}
