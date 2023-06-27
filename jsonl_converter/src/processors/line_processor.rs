//! This module contains the functionality to process a lines of a file one at
//! a time without reading the entire file into memory at once.
//!
//! Specifically, this is used to convert JSON to JSONL format.
//!
//! It assumes that the JSON file is well formatted.

// This struct contains the functionality to process lines of a file one at a
// time to convert JSON to JSONL format. It keeps track of the brackets that
// have been opened and closed by observing the start and end of each line.
//
// # Fields
//

use crate::{
    brackets::{brackets_map, is_closing_bracket, is_opening_bracket, BracketStack},
    json_object::JSONLString,
};

pub struct LineProcessor {
    pub bracket_stack: BracketStack,
    jsonl_string: JSONLString,
}

impl LineProcessor {
    /// Creates a mew instance pf `LineProcessor`.
    pub fn new() -> Self {
        Self {
            bracket_stack: BracketStack::new(),
            jsonl_string: JSONLString::new(),
        }
    }

    /// Adds a bracket to the `bracket_stack`.
    ///
    /// # Arguments
    ///
    /// * `byte` - A character.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonl_converter::processors::line_processor::LineProcessor;
    ///
    /// let mut processor = ByteProcessor::new();
    /// processor.push_bracket(&'[');
    /// ```
    pub fn push_bracket(&mut self, byte: &char) {
        self.bracket_stack.push(&byte);
    }

    pub fn process_line(&mut self, line: &str) {
        let line = line.trim().to_owned();
        // println!("\n\nNew Line");

        // println!("process_line line: {}", line);
        let start_char = line.chars().next().unwrap();
        let end_char = self.get_end_char(&line);

        // println!("start_char: {}, end_char: {}", start_char, end_char);

        if is_closing_bracket(&start_char) {
            // println!("closing bracket for start_char: {}", start_char);
            self.bracket_stack.pop_pair(&start_char);
        }

        if is_opening_bracket(&start_char) {
            // println!("opening bracket for start_char: {}", start_char);
            self.push_bracket(&start_char);
        }

        if is_closing_bracket(&end_char) {
            // println!("closing bracket for end_char: {}", end_char);
            self.bracket_stack.pop_pair(&end_char);
        }

        if is_opening_bracket(&end_char) {
            // println!("opening bracket for end_char: {}", end_char);
            self.push_bracket(&end_char);
        }

        self.jsonl_string.push_str(&line);

        if self.should_print() {
            println!("{}", self.jsonl_string);
            self.jsonl_string.clear();
        }
    }

    /// Returns the character that ends the `line`. If the `line` ends with a
    /// comma, then the second to last character is returned.
    /// If the length of the `line` is 1, then an empty character is returned.
    fn get_end_char(&self, line: &str) -> char {
        let cleaned_line = line.trim_end_matches(',');
        if cleaned_line.len() == 1 {
            return ' ';
        }
        let last_char = line.chars().last().unwrap();
        if is_closing_bracket(&last_char) {
            // check if the bracket before is the corresponding opening bracket
            let second_to_last_char = cleaned_line.chars().rev().nth(1).unwrap();
            if brackets_map().get(&last_char) == Some(&second_to_last_char) {
                return ' '; // Cancels each other out
            }
        }
        last_char
    }

    /// Checks if the `jsonl_string` should be printed. This is the case if the
    /// `bracket_stack` is empty (except for the initial opening bracket).
    fn should_print(&mut self) -> bool {
        self.bracket_stack.len() == 1
    }
}
