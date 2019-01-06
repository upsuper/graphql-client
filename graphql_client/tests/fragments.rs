#[macro_use]
extern crate graphql_client;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/fragments/query.graphql",
    schema_path = "tests/fragments/schema.graphql"
)]
pub struct FragmentReference;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/fragments/query.graphql",
    schema_path = "tests/fragments/schema.graphql"
)]
pub struct SnakeCaseFragment;

#[test]
fn fragment_reference() {
    let valid_response = json!({
        "inFragment": "value",
    });

    let valid_fragment_reference =
        serde_json::from_value::<fragment_reference::ResponseData>(valid_response).unwrap();

    assert_eq!(
        valid_fragment_reference
            .fragment_reference
            .in_fragment
            .unwrap(),
        "value"
    );
}

#[test]
fn fragments_with_snake_case_name() {
    let valid_response = json!({
        "inFragment": "value",
    });

    let valid_fragment_reference =
        serde_json::from_value::<snake_case_fragment::ResponseData>(valid_response).unwrap();

    assert_eq!(
        valid_fragment_reference
            .snake_case_fragment
            .in_fragment
            .unwrap(),
        "value"
    );
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/fragments/query.graphql",
    schema_path = "tests/fragments/schema.graphql"
)]
pub struct RecursiveFragmentQuery;

#[test]
fn recursive_fragment() {
    use recursive_fragment_query::*;

    let _ = RecursiveFragment {
        head: "ABCD".to_string(),
        tail: Some(RecursiveFragment {
            head: "EFGH",
            tail: None,
        }),
    };
}
