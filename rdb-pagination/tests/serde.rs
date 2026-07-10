#![cfg(all(feature = "derive", feature = "serde"))]

use educe::Educe;
use rdb_pagination::{Pagination, PaginationOptions, prelude::*};
use serde_json::json;

#[test]
fn pagination_options() {
    #[derive(Debug, Clone, Educe, OrderByOptions, serde::Serialize, serde::Deserialize)]
    #[educe(Default)]
    #[orderByOptions(name = user)]
    pub struct UserOrderBy {
        #[orderByOptions((user, id), unique)]
        pub id:   OrderMethod,
        #[orderByOptions((user, name), unique)]
        pub name: OrderMethod,
    }

    let options = PaginationOptions::default()
        .page(2)
        .items_per_page(20)
        .order_by(UserOrderBy {
            id: OrderMethod::from(-2), name: OrderMethod::from(1)
        });
    let text = serde_json::to_string(&options).unwrap();
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();

    assert_eq!(
        json!({
            "page": 2,
            "items_per_page": 20,
            "order_by": {
                "id": -2,
                "name": 1,
            },
        }),
        value
    );

    let options: PaginationOptions<UserOrderBy> = serde_json::from_value(value).unwrap();

    assert_eq!(2, options.page);
    assert_eq!(20, options.items_per_page);
    assert_eq!(OrderMethod::from(-2), options.order_by.id);
    assert_eq!(OrderMethod::from(1), options.order_by.name);

    let options: PaginationOptions<UserOrderBy> = serde_json::from_str(&text).unwrap();

    assert_eq!(2, options.page);

    let options: PaginationOptions<UserOrderBy> = serde_json::from_value(json!({})).unwrap();

    assert_eq!(1, options.page);
    assert_eq!(0, options.items_per_page);
    assert_eq!(OrderMethod::default(), options.order_by.id);
    assert_eq!(OrderMethod::default(), options.order_by.name);

    let order_method = OrderMethod::<i16>::from(-1024);
    let value = serde_json::to_value(order_method).unwrap();

    assert_eq!(json!(-1024), value);
    assert_eq!(order_method, serde_json::from_value(value).unwrap());
}

#[test]
fn pagination() {
    let pagination = Pagination::new().items_per_page(20).total_items(50).page(2);
    let value = serde_json::to_value(&pagination).unwrap();

    assert_eq!(
        json!({
            "page": 2,
            "total_pages": 3,
            "items_per_page": 20,
            "total_items": 50,
        }),
        value
    );

    let pagination: Pagination = serde_json::from_value(value).unwrap();

    assert_eq!(2, pagination.get_page());
    assert_eq!(3, pagination.get_total_pages());
    assert_eq!(20, pagination.get_items_per_page());
    assert_eq!(50, pagination.get_total_items());

    assert!(
        serde_json::from_value::<Pagination>(json!({
            "page": 2,
            "total_pages": 4,
            "items_per_page": 20,
            "total_items": 50,
        }))
        .is_err()
    );
    assert!(
        serde_json::from_value::<Pagination>(json!({
            "page": 4,
            "total_pages": 3,
            "items_per_page": 20,
            "total_items": 50,
        }))
        .is_err()
    );
}
