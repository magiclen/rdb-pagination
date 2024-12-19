#![cfg(all(feature = "derive", feature = "mysql"))]

use educe::Educe;
use rdb_pagination::{prelude::*, PaginationOptions, SqlJoin, SqlOrderByComponent};

#[test]
fn component() {
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
        #[orderByOptions(("component_general_type", "code"), unique)]
        pub component_general_type_code:  OrderMethod,
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

    let order_by = ComponentOrderBy::default();

    let pagination_options =
        PaginationOptions::default().page(3).items_per_page(20).order_by(order_by);

    let mut buffer = String::new();

    #[cfg(feature = "mysql")]
    assert_eq!("LIMIT 20 OFFSET 40", pagination_options.to_mysql_limit_offset(&mut buffer));

    #[cfg(feature = "sqlite")]
    assert_eq!("LIMIT 20 OFFSET 40", pagination_options.to_sqlite_limit_offset(&mut buffer));

    buffer.clear();

    let (joins, order_by_components) = pagination_options.order_by.to_sql();

    #[cfg(feature = "mysql")]
    assert_eq!(
        "LEFT JOIN `component_type` ON `component_type`.`id` = \
         `component`.`component_type_id`\nLEFT JOIN `component_general_type` ON \
         `component_general_type`.`id` = `component_type`.`component_general_type_id`\nLEFT JOIN \
         `component_vendor` ON `component_vendor`.`id` = `component_type`.`component_vendor_id`",
        SqlJoin::format_mysql_join_clauses(&joins, &mut buffer)
    );

    #[cfg(feature = "sqlite")]
    assert_eq!(
        "LEFT JOIN `component_type` ON `component_type`.`id` = \
         `component`.`component_type_id`\nLEFT JOIN `component_general_type` ON \
         `component_general_type`.`id` = `component_type`.`component_general_type_id`\nLEFT JOIN \
         `component_vendor` ON `component_vendor`.`id` = `component_type`.`component_vendor_id`",
        SqlJoin::format_sqlite_join_clauses(&joins, &mut buffer)
    );

    buffer.clear();

    #[cfg(feature = "mysql")]
    assert_eq!(
        "ORDER BY `component_type`.`order` ASC, `component_general_type`.`order` ASC, \
         `component_type`.`component_general_type_id` ASC, `component_vendor`.`order` ASC, \
         `component_type`.`component_vendor_id` IS NOT NULL, \
         `component_type`.`component_vendor_id` ASC, `component`.`component_type_id` ASC, \
         `component`.`id` ASC",
        SqlOrderByComponent::format_mysql_order_by_components(&order_by_components, &mut buffer)
    );

    #[cfg(feature = "sqlite")]
    assert_eq!(
        "ORDER BY `component_type`.`order` ASC, `component_general_type`.`order` ASC, \
         `component_type`.`component_general_type_id` ASC, `component_vendor`.`order` ASC, \
         `component_type`.`component_vendor_id` IS NOT NULL, \
         `component_type`.`component_vendor_id` ASC, `component`.`component_type_id` ASC, \
         `component`.`id` ASC",
        SqlOrderByComponent::format_sqlite_order_by_components(&order_by_components, &mut buffer)
    );
}
