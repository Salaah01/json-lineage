//! This module contains structs that represent JSON objects and parts of JSON
//! objects.

use core::fmt;
use regex::Regex;
use std::ops::Deref;

/// This struct represents a JSONL string being built.
///
/// # Fields
///
/// * `string` - The JSONL string being built.
/// * `clean_re_pattern` - A regular expression pattern used to clean the
/// JSONL string.
pub struct JSONLString {
    string: String,
    clean_re_pattern: Regex,
}

impl Deref for JSONLString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}

impl JSONLString {
    /// Creates a new instance of `JSONLString`.
    pub fn new() -> Self {
        JSONLString {
            string: String::new(),
            clean_re_pattern: Regex::new(r"\s{0,}\n\s{0,}").unwrap(),
        }
    }

    /// Adds a character to the `string`.
    ///
    /// # Arguments
    ///
    /// * `c` - A character.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonl_converter::json_object::JSONLString;
    ///
    /// let mut jsonl_string = JSONLString::new();
    /// jsonl_string.push_char(&'a');
    /// ```
    pub fn push_char(&mut self, c: &char) {
        self.string.push(*c);
    }

    /// Adds a string to the `string`.
    ///
    /// # Arguments
    ///
    /// * `s` - A string.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonl_converter::json_object::JSONLString;
    ///
    /// let mut jsonl_string = JSONLString::new();
    /// jsonl_string.push_str(&"abc");
    /// ```
    pub fn push_str(&mut self, s: &str) {
        self.string.push_str(s);
    }

    /// Clears the `string`.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonl_converter::json_object::JSONLString;
    ///
    /// let mut jsonl_string = JSONLString::new();
    /// jsonl_string.push_str(&"abc");
    /// jsonl_string.clear();
    /// ```
    pub fn clear(&mut self) {
        self.string.clear();
    }
}

impl fmt::Display for JSONLString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = self.clean_re_pattern.replace_all(&self.string, "");
        write!(
            f,
            "{}",
            result
                .trim_start_matches(',')
                .trim_end_matches(',')
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonl_string_new_instance_is_empty() {
        let jsonl_string = JSONLString::new();
        assert_eq!(jsonl_string.string, "");
    }

    #[test]
    fn test_jsonl_string_push_char_adds_char_to_string() {
        let mut jsonl_string = JSONLString::new();
        jsonl_string.push_char(&'a');
        assert_eq!(jsonl_string.string, "a");
    }

    #[test]
    fn test_jsonl_string_push_str_adds_str_to_string() {
        let mut jsonl_string = JSONLString::new();
        jsonl_string.push_str("abc");
        assert_eq!(jsonl_string.string, "abc");
    }

    #[test]
    fn test_jsonl_string_clear_removes_all_chars_from_string() {
        let mut jsonl_string = JSONLString::new();
        jsonl_string.push_str("abc");
        jsonl_string.clear();
        assert_eq!(jsonl_string.string, "");
    }

    #[test]
    fn test_jsonl_string_display_trait_impl_returns_string_without_spaces() {
        let mut jsonl_string = JSONLString::new();
        jsonl_string.push_str("    \n{\"a\": 1}\n\"");
        assert_eq!(jsonl_string.to_string(), "{\"a\": 1}\"");
    }

    #[test]
    fn test_jsonl_string_display_removes_leading_comma() {
        let mut jsonl_string = JSONLString::new();
        jsonl_string.push_str(",\n{\"a\": 1}");
        assert_eq!(jsonl_string.to_string(), "{\"a\": 1}");
    }

    #[test]
    fn test_jsonl_len_returns_string_length() {
        let mut jsonl_string = JSONLString::new();
        jsonl_string.push_str("abc");
        assert_eq!(jsonl_string.len(), 3);
    }
}
