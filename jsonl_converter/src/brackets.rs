//! This module helps us keep track of brackets that have been opened but not
//! closed. This is useful for parsing JSON objects, which can contain nested
//! objects.

use std::collections::HashMap;

/// Checks if a character is a bracket.
///
/// # Arguments
///
/// * `c` - A character.
///
/// # Returns
///
/// * `true` if the character is a bracket.
/// * `false` if the character is not a bracket.
pub fn is_bracket(c: &str) -> bool {
    match c {
        "[" | "]" | "{" | "}" | "\"" => true,
        _ => false,
    }
}

/// Checks if a character is an opening bracket.
///
/// # Arguments
///
/// * `c` - A character.
///
/// # Returns
///
/// * `true` if the character is an opening bracket.
/// * `false` if the character is not an opening bracket.
pub fn is_opening_bracket(c: &char) -> bool {
    match c {
        '[' | '{' => true,
        _ => false,
    }
}

/// Checks if a character is a closing bracket.
///     
/// # Arguments
///
/// * `c` - A character.
///
/// # Returns
///
/// * `true` if the character is a closing bracket.
/// * `false` if the character is not a closing bracket.
pub fn is_closing_bracket(c: &char) -> bool {
    match c {
        ']' | '}' => true,
        _ => false,
    }
}

/// Returns a map of brackets with their corresponding opening and closing
/// brackets.
pub fn brackets_map() -> HashMap<char, char> {
    let mut map = HashMap::new();
    map.insert(']', '[');
    map.insert('}', '{');
    map
}

/// This struct is used to keep track of brackets that have been opened but not
/// closed.
///
/// # Fields
///
/// * `stack` - A stack of brackets that have been opened but not closed.
pub struct BracketStack {
    pub stack: Vec<char>,
    _map: HashMap<char, char>,
}

impl BracketStack {
    /// Creates a new `BracketStack`.
    pub fn new() -> Self {
        BracketStack {
            stack: vec![],
            _map: brackets_map(),
        }
    }

    /// Checks if the `BracketStack` is empty.
    ///
    /// # Returns
    ///
    /// * `true` if the `BracketStack` is empty.
    /// * `false` if the `BracketStack` is not empty.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Pushes a bracket onto the `BracketStack`.
    ///
    /// # Arguments
    ///
    /// * `c` - A bracket.
    pub fn push(&mut self, c: &char) {
        self.stack.push(c.clone());
    }

    /// Pops a bracket off of the `BracketStack`.
    ///
    /// # Arguments
    ///
    /// * `c` - A bracket.
    ///
    /// # Returns
    ///
    /// * `Some(c)` if the `BracketStack` if the popped bracket matches the
    /// corresponding opening bracket.
    ///
    /// # Panics
    ///
    /// * If the `BracketStack` is empty.
    /// * If the popped bracket does not match the corresponding opening bracket.
    pub fn pop_pair(&mut self, c: &char) -> Option<char> {
        let popped = self.stack.pop().unwrap();
        if popped == self._map[&c] {
            Some(popped)
        } else {
            panic!(
                "BracketStack::pop() called on mismatched brackets - expected {:?}, got {:?}",
                self._map[&c], popped
            );
        }
    }
}

impl Iterator for BracketStack {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl ExactSizeIterator for BracketStack {
    fn len(&self) -> usize {
        self.stack.len()
    }
}
