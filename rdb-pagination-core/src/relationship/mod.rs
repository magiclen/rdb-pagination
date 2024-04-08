mod errors;

use std::collections::HashMap;

pub use errors::*;

use crate::{ColumnName, Name, TableColumn, TableName};

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct TableColumnAttributes {
    pub(crate) table_name:          TableName,
    pub(crate) column_name:         ColumnName,
    pub(crate) real_table_name:     Option<TableName>,
    pub(crate) foreign_table_name:  TableName,
    pub(crate) foreign_column_name: ColumnName,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct Relationship {
    pub(crate) table_name:   TableName,
    pub(crate) relationship: HashMap<TableName, TableColumnAttributes>,
}

impl Relationship {
    #[inline]
    pub fn new(table_name: TableName) -> Self {
        Self {
            table_name,
            relationship: HashMap::new(),
        }
    }

    #[inline]
    pub fn join_check(
        &mut self,
        foreign: TableColumn,
        primary: TableColumn,
        real_table_name: Option<TableName>,
    ) -> Result<(), JoinError> {
        if self.relationship.contains_key(&primary.0) {
            return Err(JoinError::PrimaryDuplicate);
        }

        if foreign.0 != self.table_name && !self.relationship.contains_key(&foreign.0) {
            return Err(JoinError::ForeignNotFound);
        }

        self.join(foreign, primary, real_table_name);

        Ok(())
    }

    #[inline]
    pub fn join(
        &mut self,
        foreign: TableColumn,
        primary: TableColumn,
        real_table_name: Option<TableName>,
    ) {
        self.relationship.insert(primary.0.clone(), TableColumnAttributes {
            table_name: primary.0,
            column_name: primary.1,
            real_table_name,
            foreign_table_name: foreign.0,
            foreign_column_name: foreign.1,
        });
    }

    #[inline]
    pub fn get_related_tables<'a>(&'a self, mut table_name: &'a TableName) -> Vec<&Name> {
        let mut v = Vec::new();

        while table_name.ne(&self.table_name) {
            let attrs = self.relationship.get(table_name).unwrap();

            v.push(&attrs.foreign_table_name);

            table_name = &attrs.foreign_table_name;
        }

        v
    }
}
