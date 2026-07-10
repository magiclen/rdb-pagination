#![cfg(feature = "utoipa")]

use educe::Educe;
use rdb_pagination::prelude::*;
use serde_json::Value;
use utoipa::OpenApi;

#[test]
fn order_by_options() {
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

    /// Options for ordering archived users.
    #[derive(Debug, Clone, Educe, OrderByOptions, utoipa::ToSchema)]
    #[educe(Default)]
    #[orderByOptions(name = archived_user)]
    pub struct ArchivedUserOrderBy {
        /// Order archived users by their identifier.
        #[educe(Default = 101)]
        #[orderByOptions((archived_user, id), unique)]
        pub id: OrderMethod<i16>,
    }

    #[derive(OpenApi)]
    #[openapi(components(schemas(UserOrderBy, ArchivedUserOrderBy)))]
    struct ApiDoc;

    let document = serde_json::to_value(ApiDoc::openapi()).unwrap();
    let schemas = document["components"]["schemas"].as_object().unwrap();
    let user_order_by = &schemas["UserOrderBy"];
    let properties = user_order_by["properties"].as_object().unwrap();

    assert_eq!("Options for ordering users.", user_order_by["description"]);
    assert_eq!(2, properties.len());
    assert_eq!("Order users by their identifier.", properties["id"]["description"]);
    assert_eq!("Order users by their display name.", properties["name"]["description"]);

    let archived_properties = schemas["ArchivedUserOrderBy"]["properties"].as_object().unwrap();

    assert_eq!(
        "Order archived users by their identifier.",
        archived_properties["id"]["description"]
    );

    for property in properties.values().chain(archived_properties.values()) {
        let reference = property["$ref"].as_str().unwrap();
        let schema_name = reference.rsplit('/').next().unwrap();
        let order_method_schema: &Value = &schemas[schema_name];
        let integer_schema = order_method_schema["oneOf"]
            .as_array()
            .and_then(|schemas| schemas.first())
            .unwrap_or(order_method_schema);

        assert_eq!("integer", integer_schema["type"]);
        assert!(
            order_method_schema["description"]
                .as_str()
                .unwrap()
                .starts_with("An integer value for ordering.")
        );
    }
}
