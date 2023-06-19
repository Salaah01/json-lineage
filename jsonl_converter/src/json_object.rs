//! This module contains structs that represent JSON objects and parts of JSON
//! objects.

use std::collections::HashMap;

/// This struct represents a section of a JSON object.
pub struct JSONElement {
    pub value: JSONValue,
}

impl JSONElement {
    /// Creates a new `JSONElement` from a `JSONValue`.
    pub fn new(value: JSONValue) -> Self {
        JSONElement { value }
    }
}

/// This enum represents all variants of a JSON value.
pub enum JSONValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Array(Vec<JSONValue>),
    Object(HashMap<String, JSONValue>),
}

/// Represents the overall type of a JSON object.
#[derive(Debug, PartialEq)]
pub enum JSONType {
    List,
    Object,
}

impl JSONType {
    /// Returns the `JSONType` of a `JSONElement`.
    pub fn from_element(element: &JSONElement) -> Self {
        match element.value {
            JSONValue::Array(_) => JSONType::List,
            JSONValue::Object(_) => JSONType::Object,
            _ => panic!("JSONType::from_element() called on non-list or non-object"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_type_from_element_list() {
        let element = JSONElement::new(JSONValue::Array(vec![]));
        assert_eq!(JSONType::from_element(&element), JSONType::List);
    }

    #[test]
    fn test_json_type_from_element_object() {
        let element = JSONElement::new(JSONValue::Object(HashMap::new()));
        assert_eq!(JSONType::from_element(&element), JSONType::Object);
    }
}
