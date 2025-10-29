use rdb_pagination_core::*;

#[test]
fn limit_offset() {
    let mut buffer = String::new();

    {
        #[allow(unused_variables)]
        let pagination_options =
            PaginationOptions {
                page: 1, items_per_page: 0, order_by: ()
            };

        #[cfg(feature = "mysql")]
        assert_eq!("", pagination_options.to_mysql_limit_offset(&mut buffer));

        #[cfg(feature = "sqlite")]
        assert_eq!("", pagination_options.to_sqlite_limit_offset(&mut buffer));

        #[cfg(feature = "mssql")]
        assert_eq!("", pagination_options.to_mssql_limit_offset(&mut buffer));

        #[cfg(feature = "mssql2008")]
        assert_eq!("", pagination_options.to_mssql2008_limit_offset("rn", &mut buffer));
    }

    buffer.clear();

    {
        #[allow(unused_variables)]
        let pagination_options =
            PaginationOptions {
                page: 1, items_per_page: 20, order_by: ()
            };

        #[cfg(feature = "mysql")]
        assert_eq!("LIMIT 20", pagination_options.to_mysql_limit_offset(&mut buffer));

        #[cfg(feature = "sqlite")]
        assert_eq!("LIMIT 20", pagination_options.to_sqlite_limit_offset(&mut buffer));

        #[cfg(feature = "mssql")]
        assert_eq!(
            "OFFSET 0 ROWS FETCH NEXT 20 ROWS ONLY",
            pagination_options.to_mssql_limit_offset(&mut buffer)
        );

        #[cfg(feature = "mssql2008")]
        assert_eq!(
            "WHERE [rn] <= 20",
            pagination_options.to_mssql2008_limit_offset("rn", &mut buffer)
        );
    }

    buffer.clear();

    {
        #[allow(unused_variables)]
        let pagination_options =
            PaginationOptions {
                page: 3, items_per_page: 20, order_by: ()
            };

        #[cfg(feature = "mysql")]
        assert_eq!("LIMIT 20 OFFSET 40", pagination_options.to_mysql_limit_offset(&mut buffer));

        #[cfg(feature = "sqlite")]
        assert_eq!("LIMIT 20 OFFSET 40", pagination_options.to_sqlite_limit_offset(&mut buffer));

        #[cfg(feature = "mssql")]
        assert_eq!(
            "OFFSET 40 ROWS FETCH NEXT 20 ROWS ONLY",
            pagination_options.to_mssql_limit_offset(&mut buffer)
        );

        #[cfg(feature = "mssql2008")]
        assert_eq!(
            "WHERE [rn] BETWEEN 41 AND 60",
            pagination_options.to_mssql2008_limit_offset("rn", &mut buffer)
        );
    }
}

#[test]
fn pagination() {
    let pagination = Pagination::new().items_per_page(20).total_items(50).page(5);

    assert_eq!(3, pagination.get_total_pages());
    assert_eq!(3, pagination.get_page());
}

#[test]
fn order_by() {
    let mut relationship = Relationship::new(Name::Static("component"));

    #[allow(clippy::type_complexity)]
    let joins = [
        (
            (Name::Static("component"), Name::Static("component_type_id")),
            (Name::Static("component_type"), Name::Static("id")),
            None,
        ),
        (
            (Name::Static("component_type"), Name::Static("component_general_type_id")),
            (Name::Static("component_general_type"), Name::Static("id")),
            None,
        ),
        (
            (Name::Static("component_type"), Name::Static("component_vendor_id")),
            (Name::Static("component_vendor"), Name::Static("id")),
            None,
        ),
    ];

    for (foreign, primary, real_table_name) in joins {
        relationship.join_check(foreign, primary, real_table_name).unwrap();
    }

    {
        let order_options = [
            ((Name::Static("component_general_type"), Name::Static("id")), true, 103i8),
            ((Name::Static("component_general_type"), Name::Static("name")), true, 0),
            ((Name::Static("component_general_type"), Name::Static("code")), true, 0),
            ((Name::Static("component_general_type"), Name::Static("order")), false, 102),
            ((Name::Static("component_vendor"), Name::Static("id")), true, 105),
            ((Name::Static("component_vendor"), Name::Static("name")), true, 0),
            ((Name::Static("component_vendor"), Name::Static("order")), false, 104),
            ((Name::Static("component_type"), Name::Static("id")), true, 106),
            ((Name::Static("component_type"), Name::Static("order")), false, 101),
            ((Name::Static("component"), Name::Static("id")), true, 107),
        ];

        // check
        {
            let mut order_builder: OrderBuilder<i8> =
                OrderBuilder::new(relationship.clone(), order_options.len());

            for (table_column, unique, _) in order_options.iter() {
                order_builder
                    .add_order_option_check(
                        (table_column.0.clone(), table_column.1.clone()),
                        *unique,
                    )
                    .unwrap();
            }
        }

        let mut order_builder = OrderBuilder::new(relationship.clone(), order_options.len());

        for (table_column, unique, order_method) in order_options.iter() {
            order_builder.add_order_option(
                (table_column.0.clone(), table_column.1.clone()),
                *unique,
                NullStrategy::Default,
                (*order_method).into(),
            );
        }

        #[allow(unused_variables)]
        let (joins, order_by_components) = order_builder.build();

        let mut buffer = String::new();

        #[cfg(feature = "mysql")]
        assert_eq!(
            "LEFT JOIN `component_type` ON `component_type`.`id` = \
             `component`.`component_type_id`\nLEFT JOIN `component_general_type` ON \
             `component_general_type`.`id` = `component_type`.`component_general_type_id`\nLEFT \
             JOIN `component_vendor` ON `component_vendor`.`id` = \
             `component_type`.`component_vendor_id`",
            SqlJoin::format_mysql_join_clauses(&joins, &mut buffer)
        );

        #[cfg(feature = "sqlite")]
        assert_eq!(
            "LEFT JOIN `component_type` ON `component_type`.`id` = \
             `component`.`component_type_id`\nLEFT JOIN `component_general_type` ON \
             `component_general_type`.`id` = `component_type`.`component_general_type_id`\nLEFT \
             JOIN `component_vendor` ON `component_vendor`.`id` = \
             `component_type`.`component_vendor_id`",
            SqlJoin::format_sqlite_join_clauses(&joins, &mut buffer)
        );

        #[cfg(any(feature = "mssql", feature = "mssql2008"))]
        assert_eq!(
            "LEFT JOIN [component_type] ON [component_type].[id] = \
             [component].[component_type_id]\nLEFT JOIN [component_general_type] ON \
             [component_general_type].[id] = [component_type].[component_general_type_id]\nLEFT \
             JOIN [component_vendor] ON [component_vendor].[id] = \
             [component_type].[component_vendor_id]",
            SqlJoin::format_mssql_join_clauses(&joins, &mut buffer)
        );

        buffer.clear();

        #[cfg(feature = "mysql")]
        assert_eq!(
            "ORDER BY `component_type`.`order` ASC, `component_general_type`.`order` ASC, \
             `component_type`.`component_general_type_id` ASC, `component_vendor`.`order` ASC, \
             `component_type`.`component_vendor_id` ASC, `component`.`component_type_id` ASC, \
             `component`.`id` ASC",
            SqlOrderByComponent::format_mysql_order_by_components(
                &order_by_components,
                &mut buffer
            )
        );

        #[cfg(feature = "sqlite")]
        assert_eq!(
            "ORDER BY `component_type`.`order` ASC, `component_general_type`.`order` ASC, \
             `component_type`.`component_general_type_id` ASC, `component_vendor`.`order` ASC, \
             `component_type`.`component_vendor_id` ASC, `component`.`component_type_id` ASC, \
             `component`.`id` ASC",
            SqlOrderByComponent::format_sqlite_order_by_components(
                &order_by_components,
                &mut buffer
            )
        );

        #[cfg(any(feature = "mssql", feature = "mssql2008"))]
        assert_eq!(
            "ORDER BY [component_type].[order] ASC, [component_general_type].[order] ASC, \
             [component_type].[component_general_type_id] ASC, [component_vendor].[order] ASC, \
             [component_type].[component_vendor_id] ASC, [component].[component_type_id] ASC, \
             [component].[id] ASC",
            SqlOrderByComponent::format_mssql_order_by_components(
                &order_by_components,
                &mut buffer
            )
        );
    }

    {
        let order_options = [
            ((Name::Static("component_general_type"), Name::Static("id")), true, 103i8),
            ((Name::Static("component_general_type"), Name::Static("name")), true, 0),
            ((Name::Static("component_general_type"), Name::Static("code")), true, 0),
            ((Name::Static("component_general_type"), Name::Static("order")), false, 102),
            ((Name::Static("component_vendor"), Name::Static("id")), true, 105),
            ((Name::Static("component_vendor"), Name::Static("name")), true, 0),
            ((Name::Static("component_vendor"), Name::Static("order")), false, 104),
            ((Name::Static("component_type"), Name::Static("id")), true, 106),
            ((Name::Static("component_type"), Name::Static("order")), false, 101),
            ((Name::Static("component"), Name::Static("id")), true, -1),
        ];

        // check
        {
            let mut order_builder: OrderBuilder<i8> =
                OrderBuilder::new(relationship.clone(), order_options.len());

            for (table_column, unique, _) in order_options.iter() {
                order_builder
                    .add_order_option_check(
                        (table_column.0.clone(), table_column.1.clone()),
                        *unique,
                    )
                    .unwrap();
            }
        }

        let mut order_builder = OrderBuilder::new(relationship.clone(), order_options.len());

        for (table_column, unique, order_method) in order_options.iter() {
            order_builder.add_order_option(
                (table_column.0.clone(), table_column.1.clone()),
                *unique,
                NullStrategy::Default,
                (*order_method).into(),
            );
        }

        #[allow(unused_variables)]
        let (joins, order_by_components) = order_builder.build();

        let mut buffer = String::new();

        #[cfg(feature = "mysql")]
        assert_eq!("", SqlJoin::format_mysql_join_clauses(&joins, &mut buffer));

        #[cfg(feature = "sqlite")]
        assert_eq!("", SqlJoin::format_sqlite_join_clauses(&joins, &mut buffer));

        buffer.clear();

        #[cfg(feature = "mysql")]
        assert_eq!(
            "ORDER BY `component`.`id` DESC",
            SqlOrderByComponent::format_mysql_order_by_components(
                &order_by_components,
                &mut buffer
            )
        );

        #[cfg(feature = "sqlite")]
        assert_eq!(
            "ORDER BY `component`.`id` DESC",
            SqlOrderByComponent::format_sqlite_order_by_components(
                &order_by_components,
                &mut buffer
            )
        );

        #[cfg(any(feature = "mssql", feature = "mssql2008"))]
        assert_eq!(
            "ORDER BY [component].[id] DESC",
            SqlOrderByComponent::format_mssql_order_by_components(
                &order_by_components,
                &mut buffer
            )
        );
    }

    {
        let order_options = [
            ((Name::Static("component_general_type"), Name::Static("id")), true, 103i8),
            ((Name::Static("component_general_type"), Name::Static("name")), true, 0),
            ((Name::Static("component_general_type"), Name::Static("code")), true, 0),
            ((Name::Static("component_general_type"), Name::Static("order")), false, 102),
            ((Name::Static("component_vendor"), Name::Static("id")), true, 105),
            ((Name::Static("component_vendor"), Name::Static("name")), true, 0),
            ((Name::Static("component_vendor"), Name::Static("order")), false, 104),
            ((Name::Static("component_type"), Name::Static("id")), true, 1),
            ((Name::Static("component_type"), Name::Static("order")), false, 101),
            ((Name::Static("component"), Name::Static("id")), true, 107),
        ];

        // check
        {
            let mut order_builder: OrderBuilder<i8> =
                OrderBuilder::new(relationship.clone(), order_options.len());

            for (table_column, unique, _) in order_options.iter() {
                order_builder
                    .add_order_option_check(
                        (table_column.0.clone(), table_column.1.clone()),
                        *unique,
                    )
                    .unwrap();
            }
        }

        let mut order_builder = OrderBuilder::new(relationship.clone(), order_options.len());

        for (table_column, unique, order_method) in order_options.iter() {
            order_builder.add_order_option(
                (table_column.0.clone(), table_column.1.clone()),
                *unique,
                NullStrategy::Default,
                (*order_method).into(),
            );
        }

        #[allow(unused_variables)]
        let (joins, order_by_components) = order_builder.build();

        let mut buffer = String::new();

        #[cfg(feature = "mysql")]
        assert_eq!("", SqlJoin::format_mysql_join_clauses(&joins, &mut buffer));

        #[cfg(feature = "sqlite")]
        assert_eq!("", SqlJoin::format_sqlite_join_clauses(&joins, &mut buffer));

        buffer.clear();

        #[cfg(feature = "mysql")]
        assert_eq!(
            "ORDER BY `component`.`component_type_id` ASC, `component`.`id` ASC",
            SqlOrderByComponent::format_mysql_order_by_components(
                &order_by_components,
                &mut buffer
            )
        );

        #[cfg(feature = "sqlite")]
        assert_eq!(
            "ORDER BY `component`.`component_type_id` ASC, `component`.`id` ASC",
            SqlOrderByComponent::format_sqlite_order_by_components(
                &order_by_components,
                &mut buffer
            )
        );

        #[cfg(any(feature = "mssql", feature = "mssql2008"))]
        assert_eq!(
            "ORDER BY [component].[component_type_id] ASC, [component].[id] ASC",
            SqlOrderByComponent::format_mssql_order_by_components(
                &order_by_components,
                &mut buffer
            )
        );
    }

    {
        let order_options = [
            ((Name::Static("component_general_type"), Name::Static("id")), true, 103i8),
            ((Name::Static("component_general_type"), Name::Static("name")), true, 0),
            ((Name::Static("component_general_type"), Name::Static("code")), true, 0),
            ((Name::Static("component_general_type"), Name::Static("order")), false, 102),
            ((Name::Static("component_vendor"), Name::Static("id")), true, 105),
            ((Name::Static("component_vendor"), Name::Static("name")), true, 0),
            ((Name::Static("component_vendor"), Name::Static("order")), false, 104),
            ((Name::Static("component_type"), Name::Static("id")), true, 106),
            ((Name::Static("component_type"), Name::Static("order")), false, 101),
            ((Name::Static("component"), Name::Static("id")), true, 107),
        ];

        let mut order_builder = OrderBuilder::new(relationship.clone(), order_options.len());

        for (table_column, unique, order_method) in order_options.iter() {
            order_builder.add_order_option(
                (table_column.0.clone(), table_column.1.clone()),
                *unique,
                NullStrategy::Default,
                (*order_method).into(),
            );
        }

        let (mut joins, _) = order_builder.build();

        let result = joins.add_join(SqlJoin {
            other_table_name:  Name::Static("component_type"),
            other_column_name: Name::Static("id"),
            real_table_name:   None,
            using_table_name:  Name::Static("component"),
            using_column_name: Name::Static("component_type_id"),
        });
        assert!(matches!(result, Ok(false)));

        let result = joins.add_join(SqlJoin {
            other_table_name:  Name::Static("component_type2"),
            other_column_name: Name::Static("id"),
            real_table_name:   None,
            using_table_name:  Name::Static("component"),
            using_column_name: Name::Static("component_type_id"),
        });
        assert!(matches!(result, Ok(true)));

        let result = joins.add_join(SqlJoin {
            other_table_name:  Name::Static("component_type"),
            other_column_name: Name::Static("id"),
            real_table_name:   None,
            using_table_name:  Name::Static("component"),
            using_column_name: Name::Static("id"),
        });
        assert!(result.is_err());
    }
}

#[test]
fn null_strategy() {
    let relationship = Relationship::new(Name::Static("component_general_type"));

    let order_options = [
        (
            (Name::Static("component_general_type"), Name::Static("id")),
            true,
            NullStrategy::Last,
            102i8,
        ),
        (
            (Name::Static("component_general_type"), Name::Static("name")),
            true,
            NullStrategy::Default,
            0,
        ),
        (
            (Name::Static("component_general_type"), Name::Static("code")),
            true,
            NullStrategy::Last,
            0,
        ),
        (
            (Name::Static("component_general_type"), Name::Static("order")),
            false,
            NullStrategy::First,
            101,
        ),
    ];

    let mut order_builder = OrderBuilder::new(relationship.clone(), order_options.len());

    for (table_column, unique, null_strategy, order_method) in order_options.into_iter() {
        order_builder.add_order_option(
            (table_column.0, table_column.1),
            unique,
            null_strategy,
            order_method.into(),
        );
    }

    #[allow(unused_variables)]
    let (_joins, order_by_components) = order_builder.build();

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    let mut buffer = String::new();

    #[cfg(feature = "mysql")]
    assert_eq!(
        "ORDER BY `component_general_type`.`order` IS NOT NULL, `component_general_type`.`order` \
         ASC, `component_general_type`.`id` IS NULL, `component_general_type`.`id` ASC",
        SqlOrderByComponent::format_mysql_order_by_components(&order_by_components, &mut buffer)
    );

    #[cfg(feature = "sqlite")]
    assert_eq!(
        "ORDER BY `component_general_type`.`order` IS NOT NULL, `component_general_type`.`order` \
         ASC, `component_general_type`.`id` IS NULL, `component_general_type`.`id` ASC",
        SqlOrderByComponent::format_sqlite_order_by_components(&order_by_components, &mut buffer)
    );

    #[cfg(any(feature = "mssql", feature = "mssql2008"))]
    assert_eq!(
        "ORDER BY CASE WHEN [component_general_type].[order] IS NULL THEN 0 ELSE 1 END, \
         [component_general_type].[order] ASC, CASE WHEN [component_general_type].[id] IS NULL \
         THEN 1 ELSE 0 END, [component_general_type].[id] ASC",
        SqlOrderByComponent::format_mssql_order_by_components(&order_by_components, &mut buffer)
    );
}
