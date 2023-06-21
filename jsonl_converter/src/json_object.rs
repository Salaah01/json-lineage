//! This module contains structs that represent JSON objects and parts of JSON
//! objects.

use core::fmt;
use regex::Regex;

pub struct JSONLString {
    string: String,
    clean_re_pattern: Regex,
}

impl JSONLString {
    pub fn new() -> Self {
        JSONLString {
            string: String::new(),
            clean_re_pattern: Regex::new(r"\s{0,}\n\s{0,}").unwrap(),
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
}
