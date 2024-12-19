mod errors;

use std::collections::HashSet;

pub use errors::*;

use crate::{
    NullStrategy, OrderMethod, OrderMethodValue, OrderType, Relationship, SqlJoin,
    SqlOrderByComponent, TableColumn,
};

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct OrderBuilder<T: OrderMethodValue = i8> {
    relationship:  Relationship,
    order_options: Vec<(TableColumn, bool, NullStrategy, OrderMethod<T>)>,
}

impl<T: OrderMethodValue> OrderBuilder<T> {
    #[inline]
    pub fn new(relationship: Relationship, capacity: usize) -> Self {
        Self {
            relationship,
            order_options: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn add_order_option_check(
        &mut self,
        mut table_column: TableColumn,
        unique: bool,
    ) -> Result<(), OrderOptionError> {
        if let Some(attr) = self.relationship.relationship.get(&table_column.0) {
            if attr.column_name == table_column.1 {
                table_column = (attr.foreign_table_name.clone(), attr.foreign_column_name.clone());
            }
        } else if self.relationship.table_name == table_column.0 {
            // do nothing
        } else {
            return Err(OrderOptionError::TableNotRecognized);
        }

        if self.order_options.iter().any(|(key, ..)| key == &table_column) {
            return Err(OrderOptionError::TableColumnDuplicate);
        }

        self.order_options.push((
            table_column,
            unique,
            NullStrategy::Default,
            OrderMethod(T::one()),
        ));

        Ok(())
    }

    #[inline]
    pub fn add_order_option(
        &mut self,
        table_column: TableColumn,
        unique: bool,
        null_strategy: NullStrategy,
        order_method: OrderMethod<T>,
    ) {
        if order_method.0 != T::zero() {
            let unique = unique
                || self.relationship.relationship.iter().any(|(_, value)| {
                    value.table_name == table_column.0 && value.column_name == table_column.1
                });

            self.order_options.push((table_column, unique, null_strategy, order_method));
        }
    }

    pub fn build(mut self) -> (Vec<SqlJoin>, Vec<SqlOrderByComponent>) {
        self.order_options.sort_by(|(_, _, _, a), (_, _, _, b)| a.0.abs().cmp(&b.0.abs()));

        {
            // remove unnecessary options

            let mut unique_set = HashSet::new();

            let mut i = 0;

            while i < self.order_options.len() {
                let remove = {
                    let ((table_name, _), unique, ..) = &self.order_options[i];

                    let mut remove = false;

                    if unique_set.contains(table_name) {
                        remove = true
                    } else {
                        let related_table_names = self.relationship.get_related_tables(table_name);

                        for related_table_name in related_table_names {
                            if unique_set.contains(related_table_name) {
                                remove = true;
                                break;
                            }
                        }

                        if *unique {
                            unique_set.insert(table_name.clone());
                        }
                    }

                    remove
                };

                if remove {
                    self.order_options.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        {
            // adjust options (check primary and foreign)
            for (table_column, ..) in self.order_options.iter_mut() {
                if let Some(attr) = self.relationship.relationship.get(&table_column.0) {
                    if attr.column_name == table_column.1 {
                        *table_column =
                            (attr.foreign_table_name.clone(), attr.foreign_column_name.clone());
                    }
                }
            }
        }

        // fetch OrderType

        let v = self
            .order_options
            .into_iter()
            .map(|((table_name, column_name), _, null_strategy, order_method)| {
                (table_name, column_name, null_strategy, OrderType::from_order_method(order_method))
            })
            .collect::<Vec<_>>();

        // generate sql tokens

        let mut joined = HashSet::new();
        let mut sql_joins = Vec::new();
        let mut sql_order_by_units = Vec::new();

        for (table_name, column_name, null_strategy, order_type) in v {
            let related_table_names = self.relationship.get_related_tables(&table_name);

            for related_table_name in
                related_table_names.into_iter().rev().chain([&table_name].into_iter())
            {
                if joined.contains(related_table_name) {
                    continue;
                }

                joined.insert(related_table_name.clone());

                if let Some(attrs) = self.relationship.relationship.get(related_table_name) {
                    sql_joins.push(SqlJoin::from_table_column_attributes(attrs));
                }
            }

            sql_order_by_units.push(SqlOrderByComponent {
                table_name,
                column_name,
                order_type,
                null_strategy,
            });
        }

        (sql_joins, sql_order_by_units)
    }
}
