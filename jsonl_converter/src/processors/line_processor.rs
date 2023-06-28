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
    pub jsonl_string: JSONLString,
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
    /// let mut processor = LineProcessor::new();
    /// processor.push_bracket(&'[');
    /// assert_eq!(processor.bracket_stack.len(), 1);
    /// ```
    pub fn push_bracket(&mut self, byte: &char) {
        self.bracket_stack.push(&byte);
    }

    /// Processes a line of a file. Whilst processing the line, it checks if
    /// their are any brackets. Keeping a track of the brackets allows it to
    /// determine when a JSON object has been fully read.
    /// If the JSON object has been fully read, then the JSON object is printed
    /// to stdout.
    ///
    /// # Arguments
    ///
    /// * `line` - A line of a file.
    pub fn process_line(&mut self, line: &str) {
        let line = line.trim().to_owned();

        let start_char = line.chars().next().unwrap();
        let end_char = self.get_end_char(&line);

        if is_opening_bracket(&start_char) {
            self.push_bracket(&start_char);
        }

        if is_closing_bracket(&end_char) {
            self.bracket_stack.pop_pair(&end_char);
        }

        if is_opening_bracket(&end_char) {
            self.push_bracket(&end_char);
        }

        if is_closing_bracket(&start_char) {
            self.bracket_stack.pop_pair(&start_char);
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
        let last_char = cleaned_line.chars().last().unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_returns_processor_with_empty_attrs() {
        let processor = LineProcessor::new();
        assert_eq!(processor.bracket_stack.len(), 0);
        assert_eq!(processor.jsonl_string.len(), 0);
    }

    #[test]
    fn test_get_end_char_returns_empty_char_when_line_is_one_char() {
        let processor = LineProcessor::new();
        let line = "{";
        assert_eq!(processor.get_end_char(&line), ' ');
    }

    #[test]
    fn test_get_end_char_returns_second_to_last_char_if_ends_with_comma() {
        let processor = LineProcessor::new();
        let line = "  \"name\": \"John\",";
        assert_eq!(processor.get_end_char(&line), '"');
    }

    #[test]
    fn test_get_end_char_returns_last_char_if_does_not_end_with_comma() {
        let processor = LineProcessor::new();
        let line = "  \"name\": \"John\"";
        assert_eq!(processor.get_end_char(&line), '"');
    }

    #[test]
    fn test_get_end_char_returns_empty_str_if_len_2_and_last_char_is_comma() {
        let processor = LineProcessor::new();
        let line = "{,";
        assert_eq!(processor.get_end_char(&line), ' ');
    }

    #[test]
    fn test_get_end_char_returns_str_if_last_two_open_and_close() {
        let processor = LineProcessor::new();
        let line = "cars: [],";
        assert_eq!(processor.get_end_char(&line), ' ');
    }

    #[test]
    fn test_process_line_returns_object_when_filled() {
        let mut processor = LineProcessor::new();

        processor.process_line("[");
        assert_eq!(processor.bracket_stack.stack, vec!['[']);

        processor.process_line("  {");
        assert_eq!(processor.should_print(), false);
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{']);

        processor.process_line("    \"name\": \"John\",");
        assert_eq!(processor.should_print(), false);
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{']);

        processor.process_line("    \"age\": 30,");
        assert_eq!(processor.should_print(), false);
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{']);

        processor.process_line("    \"cars\": [");
        assert_eq!(processor.should_print(), false);
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{', '[']);

        processor.process_line("    \"cars\": [");
        assert_eq!(processor.should_print(), false);
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{', '[', '[']);

        processor.process_line(
            "      { \"name\": \"Ford\", \"models\": [ \"Fiesta\", \"Focus\", \"Mustang\" ] },",
        );
        assert_eq!(processor.should_print(), false);
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{', '[', '[']);

        processor
            .process_line("      { \"name\": \"BMW\", \"models\": [ \"320\", \"X3\", \"X5\" ] },");
        assert_eq!(processor.should_print(), false);
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{', '[', '[']);

        processor.process_line("      { \"name\": \"Fiat\", \"models\": [ \"500\", \"Panda\" ] }");
        assert_eq!(processor.should_print(), false);
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{', '[', '[']);

        processor.process_line("    ]");
        assert_eq!(processor.should_print(), false);
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{', '[']);

        processor.process_line("  ]");
        assert_eq!(processor.should_print(), false);
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{']);

        processor.process_line("}");
        assert_eq!(processor.should_print(), true);
        assert_eq!(processor.bracket_stack.stack, vec!['[']);
    }
}
