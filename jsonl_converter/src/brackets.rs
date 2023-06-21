//! This module helps us keep track of brackets that have been opened but not
//! closed. This is useful for parsing JSON objects, which can contain nested
//! objects.

use std::collections::HashMap;

/// Checks if a character is an opening bracket. Note: this function does not
/// consider '(' to be an opening bracket because it is not used in JSON.
///
/// # Arguments
///
/// * `c` - A character.
///
/// # Returns
///
/// * `true` if the character is an opening bracket.
/// * `false` if the character is not an opening bracket.
///
/// # Examples
///
/// ```
/// use jsonl_converter::brackets::is_opening_bracket;
///
/// assert_eq!(is_opening_bracket(&'['), true);
/// assert_eq!(is_opening_bracket(&'a'), false);
/// assert_eq!(is_opening_bracket(&'('), false);
/// assert_eq!(is_opening_bracket(&'{'), true);
/// ```
pub fn is_opening_bracket(c: &char) -> bool {
    match c {
        '[' | '{' => true,
        _ => false,
    }
}

/// Checks if a character is a closing bracket. Note: this function does not
/// consider ')' to be a closing bracket because it is not used in JSON.
///     
/// # Arguments
///
/// * `c` - A character.
///
/// # Returns
///
/// * `true` if the character is a closing bracket.
/// * `false` if the character is not a closing bracket.
///
/// # Examples
///
/// ```
/// use jsonl_converter::brackets::is_closing_bracket;
///
/// assert_eq!(is_closing_bracket(&']'), true);
/// assert_eq!(is_closing_bracket(&'a'), false);
/// assert_eq!(is_closing_bracket(&')'), false);
/// assert_eq!(is_closing_bracket(&'}'), true);
/// 
/// ```
pub fn is_closing_bracket(c: &char) -> bool {
    match c {
        ']' | '}' => true,
        _ => false,
    }
}

/// Returns a map of brackets with their corresponding opening and closing
/// brackets.
///
/// # Returns
///
/// * A map of brackets with their corresponding opening and closing brackets.
///
/// # Examples
///
/// ```
/// use jsonl_converter::brackets::brackets_map;
///
/// let map = brackets_map();
///
/// assert_eq!(map.get(&']'), Some(&'['));
/// assert_eq!(map.get(&'}'), Some(&'{'));
/// ```
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

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_is_opening_bracket_returns_true_for_opening_bracket() {
        assert_eq!(is_opening_bracket(&'['), true);
        assert_eq!(is_opening_bracket(&'{'), true);
        assert_eq!(is_opening_bracket(&']'), false);
        assert_eq!(is_opening_bracket(&'}'), false);
    }

    #[test]
    fn test_is_closing_bracket_returns_true_for_closing_bracket() {
        assert_eq!(is_closing_bracket(&']'), true);
        assert_eq!(is_closing_bracket(&'}'), true);
        assert_eq!(is_closing_bracket(&'['), false);
        assert_eq!(is_closing_bracket(&'{'), false);
    }

    #[test]
    fn test_brackets_map_returns_correct_map() {
        let map = brackets_map();
        assert_eq!(map[&']'], '[');
        assert_eq!(map[&'}'], '{');
    }

    #[test]
    fn test_bracket_stack_is_empty_returns_true_for_empty_bracket_stack() {
        let stack = BracketStack::new();
        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn test_bracket_stack_is_empty_returns_false_for_non_empty_bracket_stack() {
        let mut stack = BracketStack::new();
        stack.push(&'[');
        assert_eq!(stack.is_empty(), false);
    }

    #[test]
    fn test_bracket_stack_push_adds_bracket_to_stack() {
        let mut stack = BracketStack::new();
        stack.push(&'[');
        assert_eq!(stack.stack, vec!['[']);
    }

    #[test]
    fn test_bracket_stack_pop_pair_returns_correct_bracket() {
        let mut stack = BracketStack::new();
        stack.push(&'[');
        assert_eq!(stack.pop_pair(&']'), Some('['));
    }

    #[test]
    #[should_panic]
    fn test_bracket_stack_pop_pair_panics_on_mismatched_brackets() {
        let mut stack = BracketStack::new();
        stack.push(&'[');
        stack.pop_pair(&'{');
    }

    #[test]
    fn test_bracket_stack_iterator() {
        let mut stack = BracketStack::new();
        stack.push(&'[');
        stack.push(&'{');
        stack.push(&'}');
        stack.push(&']');
        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(']'));
        assert_eq!(iter.next(), Some('}'));
        assert_eq!(iter.next(), Some('{'));
        assert_eq!(iter.next(), Some('['));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_bracket_stack_exact_size_iterator() {
        let mut stack = BracketStack::new();
        stack.push(&'[');
        stack.push(&'{');
        stack.push(&'}');
        stack.push(&']');
        assert_eq!(stack.len(), 4);
    }
}
