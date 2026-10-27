//! Downconversion of the Hetzner OpenAPI 3.1 document to the scoped
//! OpenAPI 3.0.3 input that Progenitor accepts.
//!
//! Key order is preserved (`serde_json/preserve_order`), and removals use
//! `shift_remove`, so the converted document diffs cleanly against the
//! upstream snapshot.

use serde_json::{Map, Value};

/// Operation tags included in the generated client.
pub const RESOURCE_TAGS: &[&str] = &[
    "Actions",
    "Servers",
    "Server Types",
    "Server Actions",
    "Locations",
    "Images",
    "Image Actions",
    "SSH Keys",
    "Networks",
    "Network Actions",
    "Firewalls",
    "Firewall Actions",
    "Primary IPs",
    "Primary IP Actions",
    "Pricing",
];

const HTTP_METHODS: &[&str] = &[
    "get", "put", "post", "delete", "options", "head", "patch", "trace",
];

const PATH_ITEM_FIELDS: &[&str] = &["summary", "description", "servers", "parameters"];

/// JSON Schema 2020-12 keywords with no OpenAPI 3.0 equivalent. The Hetzner
/// document does not rely on dynamic references.
const DROPPED_KEYWORDS: &[&str] = &[
    "$schema",
    "$id",
    "$anchor",
    "$dynamicAnchor",
    "$dynamicRef",
    "$comment",
];

const COMPONENT_SECTIONS: &[&str] = &["schemas", "parameters", "responses", "requestBodies"];

/// Counts reported after a successful conversion.
#[derive(Debug, PartialEq, Eq)]
pub struct Summary {
    pub operations: usize,
    pub paths: usize,
}

/// Convert an OpenAPI 3.1 document in place, keeping only operations tagged
/// with one of [`RESOURCE_TAGS`].
pub fn downconvert(document: &mut Value) -> Result<Summary, String> {
    let root = document
        .as_object_mut()
        .ok_or("OpenAPI document must be a JSON object")?;
    match root.get("openapi").and_then(Value::as_str) {
        Some(version) if version.starts_with("3.1.") => {}
        Some(version) => return Err(format!("expected OpenAPI 3.1.x, found {version}")),
        None => return Err("OpenAPI document is missing its version".into()),
    }
    root.insert("openapi".into(), "3.0.3".into());
    root.shift_remove("jsonSchemaDialect");

    let paths = root
        .get_mut("paths")
        .and_then(Value::as_object_mut)
        .ok_or("OpenAPI document has no paths object")?;
    let (mut scoped, operations) = scope_paths(paths);
    scoped.iter_mut().for_each(|(_, item)| convert_schema(item));
    let path_count = scoped.len();
    *paths = scoped;

    if let Some(Value::Array(tags)) = root.get_mut("tags") {
        tags.retain(|tag| {
            tag.get("name")
                .and_then(Value::as_str)
                .is_some_and(|name| RESOURCE_TAGS.contains(&name))
        });
    }

    let components = root
        .get_mut("components")
        .and_then(Value::as_object_mut)
        .ok_or("OpenAPI document has no components object")?;
    for section in COMPONENT_SECTIONS {
        let value = components
            .entry(*section)
            .or_insert_with(|| Value::Object(Map::new()));
        convert_schema(value);
    }

    Ok(Summary {
        operations,
        paths: path_count,
    })
}

fn scope_paths(paths: &Map<String, Value>) -> (Map<String, Value>, usize) {
    let mut scoped = Map::new();
    let mut operations = 0;
    for (path, item) in paths {
        let Some(item) = item.as_object() else {
            continue;
        };
        let mut kept: Map<String, Value> = item
            .iter()
            .filter(|(key, _)| key.starts_with("x-") || PATH_ITEM_FIELDS.contains(&key.as_str()))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        for (method, operation) in item {
            if is_resource_operation(operation) {
                kept.insert(method.clone(), operation.clone());
                operations += 1;
            }
        }
        if HTTP_METHODS.iter().any(|method| kept.contains_key(*method)) {
            scoped.insert(path.clone(), Value::Object(kept));
        }
    }
    (scoped, operations)
}

fn is_resource_operation(operation: &Value) -> bool {
    let Some(operation) = operation.as_object() else {
        return false;
    };
    operation.contains_key("operationId")
        && operation
            .get("tags")
            .and_then(Value::as_array)
            .is_some_and(|tags| {
                tags.iter()
                    .filter_map(Value::as_str)
                    .any(|tag| RESOURCE_TAGS.contains(&tag))
            })
}

/// Rewrite JSON Schema 2020-12 constructs into their OpenAPI 3.0 forms,
/// children first.
pub fn convert_schema(node: &mut Value) {
    match node {
        Value::Array(items) => items.iter_mut().for_each(convert_schema),
        Value::Object(object) => {
            object.values_mut().for_each(convert_schema);
            for keyword in DROPPED_KEYWORDS {
                object.shift_remove(*keyword);
            }
            convert_type(object);
            if let Some(value) = object.shift_remove("const") {
                object.insert("enum".into(), Value::Array(vec![value]));
            }
            convert_exclusive_bound(object, "exclusiveMinimum", "minimum");
            convert_exclusive_bound(object, "exclusiveMaximum", "maximum");
            convert_examples(object);
        }
        _ => {}
    }
}

/// `type: [T, "null"]` becomes `type: T, nullable: true`; several non-null
/// types become an `anyOf` that still admits null.
fn convert_type(object: &mut Map<String, Value>) {
    let Some(Value::Array(kinds)) = object.get("type") else {
        return;
    };
    let non_null: Vec<Value> = kinds
        .iter()
        .filter(|kind| kind.as_str() != Some("null"))
        .cloned()
        .collect();
    match non_null.len() {
        0 => {}
        1 => {
            let kind = non_null.into_iter().next().unwrap_or_default();
            object.insert("type".into(), kind);
            object.insert("nullable".into(), Value::Bool(true));
        }
        _ => {
            object.shift_remove("type");
            let any_of = non_null
                .into_iter()
                .chain(std::iter::once(Value::from("null")))
                .map(|kind| {
                    let mut schema = Map::new();
                    schema.insert("type".into(), kind);
                    Value::Object(schema)
                })
                .collect();
            object.insert("anyOf".into(), Value::Array(any_of));
        }
    }
}

/// In JSON Schema 2020-12 exclusive bounds carry a number; OpenAPI 3.0 uses
/// a boolean next to `minimum`/`maximum`.
fn convert_exclusive_bound(object: &mut Map<String, Value>, exclusive: &str, bound: &str) {
    if let Some(Value::Number(value)) = object.get(exclusive) {
        let value = Value::Number(value.clone());
        object.insert(bound.into(), value);
        object.insert(exclusive.into(), Value::Bool(true));
    }
}

/// JSON Schema `examples` is an array; OpenAPI 3.0 Schema Objects allow a
/// single `example`, so keep the first unless one is already present.
/// Object-valued `examples` are OpenAPI 3.0 Media Type/Parameter example maps
/// and stay as they are.
fn convert_examples(object: &mut Map<String, Value>) {
    if !matches!(object.get("examples"), Some(Value::Array(_))) {
        return;
    }
    let Some(Value::Array(examples)) = object.shift_remove("examples") else {
        return;
    };
    if let Some(example) = examples.into_iter().next() {
        object.entry("example").or_insert(example);
    }
}

#[cfg(test)]
mod tests;
