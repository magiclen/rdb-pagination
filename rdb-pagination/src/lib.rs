/*!
# RDB Pagination

SQL query extension library for handling pagination and sorting in relational databases.

## Usage

Use `#[derive(OrderByOptions)]` to have a struct implement the `OrderByOptions` trait.

```rust
use educe::Educe;
use rdb_pagination::{prelude::*, PaginationOptions, SqlJoin, SqlOrderByComponent};

# #[cfg(feature = "derive")]
# {
#[derive(Debug, Clone, Educe, OrderByOptions)]
#[educe(Default)]
#[orderByOptions(name = component)]
#[orderByOptions(
    join((component, component_type_id), (component_type, id)),
    join((component_type, component_general_type_id), (component_general_type, id)),
    join((component_type, component_vendor_id), (component_vendor, id)),
)]
pub struct ComponentOrderBy {
    #[educe(Default = 103)]
    #[orderByOptions(("component_general_type", "id"), unique)]
    pub component_general_type_id:    OrderMethod,
    #[orderByOptions(("component_general_type", "name"), unique)]
    pub component_general_type_name:  OrderMethod,
    #[educe(Default = 102)]
    #[orderByOptions(("component_general_type", "order"))]
    pub component_general_type_order: OrderMethod,
    #[educe(Default = 105)]
    #[orderByOptions(("component_vendor", "id"), unique, nulls_first)]
    pub vendor_id:                    OrderMethod,
    #[orderByOptions(("component_vendor", "name"), unique)]
    pub vendor_name:                  OrderMethod,
    #[educe(Default = 104)]
    #[orderByOptions(("component_vendor", "order"))]
    pub vendor_order:                 OrderMethod,
    #[educe(Default = 106)]
    #[orderByOptions(("component_type", "id"), unique)]
    pub component_type_id:            OrderMethod,
    #[educe(Default = 101)]
    #[orderByOptions(("component_type", "order"))]
    pub component_type_order:         OrderMethod,
    #[educe(Default = 107)]
    #[orderByOptions(("component", "id"), unique)]
    pub id:                           OrderMethod,
}

let order_by = ComponentOrderBy {
    component_general_type_id: OrderMethod::from(-1i8),
    ..ComponentOrderBy::default()
};

let pagination_options = PaginationOptions::default().page(3).items_per_page(20).order_by(order_by);

let mut buffer = String::new();

# #[cfg(feature = "mysql")]
assert_eq!("LIMIT 20 OFFSET 40", pagination_options.to_mysql_limit_offset(&mut buffer));

# #[cfg(feature = "mssql")]
assert_eq!("OFFSET 40 ROWS FETCH NEXT 20 ROWS ONLY", pagination_options.to_mssql_limit_offset(&mut buffer));

# #[cfg(feature = "mssql2008")]
assert_eq!("WHERE [rn] BETWEEN 41 AND 60", pagination_options.to_mssql2008_limit_offset("rn", &mut buffer));

buffer.clear();

let (joins, order_by_components) = pagination_options.order_by.to_sql();

# #[cfg(feature = "mysql")]
assert_eq!(
    "LEFT JOIN `component_type` ON `component_type`.`id` = `component`.`component_type_id`\nLEFT JOIN `component_vendor` ON `component_vendor`.`id` = `component_type`.`component_vendor_id`",
    SqlJoin::format_mysql_join_clauses(&joins, &mut buffer)
);

buffer.clear();

# #[cfg(feature = "mysql")]
assert_eq!(
    "ORDER BY `component_type`.`component_general_type_id` DESC, `component_type`.`order` ASC, `component_vendor`.`order` ASC, `component_type`.`component_vendor_id` IS NOT NULL, `component_type`.`component_vendor_id` ASC, `component`.`component_type_id` ASC, `component`.`id` ASC",
    SqlOrderByComponent::format_mysql_order_by_components(&order_by_components, &mut buffer)
);
# }
```

## Serde Support

Enable the `serde` feature and add `serde` as a direct dependency with its `derive` feature to serialize and deserialize ordering options.

```toml
[dependencies]
rdb-pagination = { version = "0.3", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

`Pagination` implements serialization and validated deserialization directly, while `PaginationOptions<T>` implements each serde trait only when `T` implements the same trait.

Application-defined ordering structs must derive `serde::Serialize` and `serde::Deserialize`, while each `OrderMethod` is represented by its inner integer value.

```rust
use educe::Educe;
use rdb_pagination::{PaginationOptions, prelude::*};

# #[cfg(all(feature = "derive", feature = "serde"))]
# {
#[derive(Debug, Clone, Educe, OrderByOptions, serde::Serialize, serde::Deserialize)]
#[educe(Default)]
#[orderByOptions(name = user)]
pub struct UserOrderBy {
    #[orderByOptions((user, id), unique)]
    pub id: OrderMethod,
}

let options = PaginationOptions::default().order_by(UserOrderBy {
    id: OrderMethod::from(-1),
});
let json = serde_json::to_string(&options).unwrap();

assert_eq!(r#"{"page":1,"items_per_page":0,"order_by":{"id":-1}}"#, json);

let options: PaginationOptions<UserOrderBy> = serde_json::from_str(&json).unwrap();

assert_eq!(OrderMethod::from(-1), options.order_by.id);
# }
```

## Utoipa Support

Enable the `utoipa` feature and add `utoipa` as a direct dependency to derive `utoipa::ToSchema` for ordering options.

Struct and field documentation is included in the generated OpenAPI schema, and supported `#[schema(...)]` attributes can provide additional customization.

```rust
use educe::Educe;
use rdb_pagination::prelude::*;

# #[cfg(all(feature = "derive", feature = "utoipa"))]
# {
/// Options for ordering users.
#[derive(Debug, Clone, Educe, OrderByOptions, utoipa::ToSchema)]
#[educe(Default)]
#[orderByOptions(name = user)]
pub struct UserOrderBy {
    /// Order users by their identifier.
    #[educe(Default = 102)]
    #[orderByOptions((user, id), unique)]
    pub id:   OrderMethod,
    /// Order users by their display name.
    #[educe(Default = 101)]
    #[orderByOptions((user, name), unique)]
    pub name: OrderMethod,
}
# }
```
*/

#![cfg_attr(docsrs, feature(doc_cfg))]

pub use rdb_pagination_core::*;
#[cfg(feature = "derive")]
pub use rdb_pagination_derive::*;

/// A convenience module appropriate for glob imports.
///
/// ```rust
/// use rdb_pagination::prelude::*;
/// ```
pub mod prelude {
    pub use rdb_pagination_core::{OrderByOptions, OrderMethod, OrderMethodValue};
    #[cfg(feature = "derive")]
    pub use rdb_pagination_derive::OrderByOptions;

    #[doc(hidden)]
    pub mod rdb_pagination_prelude {
        pub use rdb_pagination_core::{
            Name, NullStrategy, OrderBuilder, Relationship, SqlJoin, SqlOrderByComponent,
        };
    }
}
