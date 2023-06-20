//! This module contains structs that represent JSON objects and parts of JSON
//! objects.

use core::fmt;
use std::collections::HashMap;

use regex::Regex;

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
    pub fn from_char(c: &char) -> Self {
        match c {
            '[' => JSONType::List,
            '{' => JSONType::Object,
            c => panic!(
                "JSONType::from_char() called on non-list or non-object ({})",
                c
            ),
        }
    }
}

pub struct JSONLString {
    string: String,
    clean_re_pattern: Regex,
}

impl JSONLString {
    pub fn new() -> Self {
        JSONLString {
            string: String::new(),
            clean_re_pattern: Regex::new(r"\n\s{1,}").unwrap(),
        }
    }

    pub fn push_char(&mut self, c: &char) {
        self.string.push(*c);
    }

    pub fn push_str(&mut self, s: &str) {
        self.string.push_str(s);
    }

    pub fn clear(&mut self) {
        self.string.clear();
    }
}

impl fmt::Display for JSONLString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = self.clean_re_pattern.replace_all(&self.string, "");
        write!(f, "{}", result.to_string().trim_start_matches(','))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_type_from_element_list() {
        let element = JSONElement::new(JSONValue::Array(vec![]));
        assert_eq!(JSONType::from_char(&element), JSONType::List);
    }

    #[test]
    fn test_json_type_from_element_object() {
        let element = JSONElement::new(JSONValue::Object(HashMap::new()));
        assert_eq!(JSONType::from_char(&element), JSONType::Object);
    }
}
