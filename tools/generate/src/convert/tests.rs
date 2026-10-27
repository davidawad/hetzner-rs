use serde_json::{json, Value};

use super::{convert_schema, downconvert, Summary};

fn parse(text: &str) -> Value {
    serde_json::from_str(text).expect("fixture is valid JSON")
}

/// Compare serialized text so key order is part of the assertion.
fn assert_same(actual: &Value, expected: &Value) {
    assert_eq!(
        serde_json::to_string_pretty(actual).unwrap(),
        serde_json::to_string_pretty(expected).unwrap()
    );
}

fn assert_schema_fixture(input: &str, expected: &str) {
    let mut value = parse(input);
    convert_schema(&mut value);
    assert_same(&value, &parse(expected));
}

#[test]
fn type_arrays_become_nullable_or_any_of() {
    assert_schema_fixture(
        include_str!("../../fixtures/type-arrays.in.json"),
        include_str!("../../fixtures/type-arrays.out.json"),
    );
}

#[test]
fn const_becomes_single_value_enum() {
    assert_schema_fixture(
        include_str!("../../fixtures/const.in.json"),
        include_str!("../../fixtures/const.out.json"),
    );
}

#[test]
fn schema_examples_become_first_example() {
    assert_schema_fixture(
        include_str!("../../fixtures/examples.in.json"),
        include_str!("../../fixtures/examples.out.json"),
    );
}

#[test]
fn exclusive_bounds_and_dialect_keywords() {
    assert_schema_fixture(
        include_str!("../../fixtures/keywords.in.json"),
        include_str!("../../fixtures/keywords.out.json"),
    );
}

#[test]
fn document_is_scoped_to_resource_tags() {
    let mut document = parse(include_str!("../../fixtures/document.in.json"));
    let summary = downconvert(&mut document).unwrap();
    assert_eq!(
        summary,
        Summary {
            operations: 1,
            paths: 1
        }
    );
    assert_same(
        &document,
        &parse(include_str!("../../fixtures/document.out.json")),
    );
}

#[test]
fn rejects_documents_that_are_not_openapi_31() {
    for document in [
        json!({"openapi": "3.0.3", "paths": {}, "components": {}}),
        json!({"swagger": "2.0"}),
        json!([]),
    ] {
        assert!(downconvert(&mut document.clone()).is_err(), "{document}");
    }
}

#[test]
fn vendored_spec_matches_checked_in_conversion() {
    let mut document = parse(include_str!("../../../../spec/cloud.spec.json"));
    downconvert(&mut document).unwrap();
    let mut converted = serde_json::to_string_pretty(&document).unwrap();
    converted.push('\n');
    assert!(
        converted == include_str!("../../../../spec/cloud.openapi-3.0.json"),
        "spec/cloud.openapi-3.0.json is stale; run scripts/regen --offline"
    );
}
