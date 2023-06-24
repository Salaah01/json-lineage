//! This module contains the functionality to process a stream of bytes to
//! convert JSON to JSONL.

use crate::{
    brackets::{is_closing_bracket, is_opening_bracket, BracketStack},
    json_object::JSONLString,
};

/// This struct contains the functionality to process a stream of bytes to
/// convert JSON to JSONL. It keeps track of the brackets that have been opened
/// but not closed, as well as the JSONL string that is being built.
///
/// # Fields
///
/// * `bracket_stack` - A stack of brackets that have been opened but not closed.
/// * `jsonl_string` - The JSONL string that is being built.
///
/// # Examples
pub struct Processor {
    bracket_stack: BracketStack,
    jsonl_string: JSONLString,
    inside_string: bool,
    last_char_escape: bool,
}

impl Processor {
    /// Creates a new instance of `Processor`.
    pub fn new() -> Self {
        Processor {
            bracket_stack: BracketStack::new(),
            jsonl_string: JSONLString::new(),
            inside_string: false,
            last_char_escape: false,
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
    /// use jsonl_converter::processor::Processor;
    ///
    /// let mut processor = Processor::new();
    /// processor.push_bracket(&'[');
    /// ```
    pub fn push_bracket(&mut self, byte: &char) {
        self.bracket_stack.push(&byte);
    }

    /// Processes a character. This function will either add the character to the
    /// `jsonl_string` or print the `jsonl_string` if the character is a closing
    /// bracket and the `bracket_stack` is empty (except for the initial
    /// opening bracket).
    ///
    /// # Arguments
    ///
    /// * `byte` - A character.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsonl_converter::processor::Processor;
    ///
    /// let mut processor = Processor::new();
    /// processor.push_bracket(&'[');
    /// processor.process_char(&'{');
    /// processor.process_char(&'a');
    /// processor.process_char(&':');
    /// processor.process_char(&'1');
    /// processor.process_char(&'}');
    /// ```
    pub fn process_char(&mut self, byte: &char) {
        match byte {
            &'"' => self.process_quote(byte),
            b if !self.inside_string && is_opening_bracket(&b) => self.process_opening_bracket(b),
            b if !self.inside_string && is_closing_bracket(&b) => self.process_closing_bracket(b),
            _ => self.process_other_char(byte),
        }

        self.last_char_escape = byte == &'\\';
    }

    /// Processes a character that is a quote. This function will add the
    /// character to the `jsonl_string` and toggle the `inside_string` flag.
    fn process_quote(&mut self, byte: &char) {
        self.jsonl_string.push_char(&byte);
        if !self.last_char_escape {
            self.inside_string = !self.inside_string;
        }
    }

    /// Processes an opening bracket by adding it to the `bracket_stack` and
    /// `jsonl_string`.
    fn process_opening_bracket(&mut self, byte: &char) {
        self.bracket_stack.push(&byte);
        self.jsonl_string.push_char(&byte);
    }

    /// Processes a closing bracket by popping the corresponding opening bracket
    /// from the `bracket_stack` and adding it to the `jsonl_string`. If the
    /// `bracket_stack` is empty (except for the initial opening bracket), the
    /// `jsonl_string` is printed and cleared.
    fn process_closing_bracket(&mut self, byte: &char) {
        self.bracket_stack.pop_pair(&byte).unwrap();

        if self.should_print() {
            self.jsonl_string.push_char(&byte);

            println!("{}", self.jsonl_string);
            self.jsonl_string.clear();
        } else {
            self.jsonl_string.push_char(&byte);
        }
    }

    /// Processes a character that is not a bracket by adding it to the
    /// `jsonl_string`.
    fn process_other_char(&mut self, byte: &char) {
        self.jsonl_string.push_char(&byte);
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
    fn test_processor_new_returns_processor_with_empty_attrs() {
        let processor = Processor::new();
        assert_eq!(processor.bracket_stack.is_empty(), true);
    }

    #[test]
    fn test_processor_push_bracket_adds_bracket_to_bracket_stack() {
        let mut processor = Processor::new();
        processor.push_bracket(&'[');
        assert_eq!(processor.bracket_stack.stack, vec!['[']);
    }

    #[test]
    fn test_processor_process_quote_pushes_quote_to_jsonl_string() {
        let mut processor = Processor::new();
        processor.process_quote(&'"');
        assert_eq!(processor.jsonl_string.to_string(), String::from("\""));
    }

    #[test]
    fn test_processor_process_quote_flips_inside_string_flag() {
        let mut processor = Processor::new();
        processor.process_quote(&'"');
        assert_eq!(processor.inside_string, true);
        processor.process_quote(&'"');
        assert_eq!(processor.inside_string, false);
    }

    #[test]
    fn test_last_char_escape_flag_flipped_on_escape_char() {
        let mut processor = Processor::new();
        processor.process_char(&'\\');
        assert_eq!(processor.last_char_escape, true);
        processor.process_char(&'a');
        assert_eq!(processor.last_char_escape, false);
    }

    #[test]
    fn test_bracket_inside_str_is_treated_as_string() {
        let mut processor = Processor::new();
        processor.process_char(&'"');
        processor.process_char(&'[');
        assert_eq!(processor.jsonl_string.to_string(), String::from("\"["));
        assert_eq!(processor.inside_string, true);
        assert_eq!(processor.bracket_stack.stack.len(), 0);
    }

    #[test]
    fn test_process_opening_bracket() {
        let mut processor = Processor::new();
        processor.process_opening_bracket(&'[');
        assert_eq!(processor.bracket_stack.stack, vec!['[']);
        assert_eq!(processor.jsonl_string.to_string(), String::from("["));
    }

    #[test]
    fn test_process_opening_bracket_recognises_entire_line_not_ready() {
        let mut processor = Processor::new();
        processor.bracket_stack.push(&'[');
        processor.bracket_stack.push(&'{');
        processor.bracket_stack.push(&'{');
        processor.jsonl_string.push_str(&"{'a': {'a': 1");
        processor.process_closing_bracket(&'}');

        assert_eq!(
            processor.jsonl_string.to_string(),
            String::from("{'a': {'a': 1}")
        );
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{']);
    }

    #[test]
    fn test_process_opening_bracket_recognises_line_is_ready() {
        let mut processor = Processor::new();
        processor.bracket_stack.push(&'[');
        processor.bracket_stack.push(&'{');
        processor.jsonl_string.push_str(&"{'a': {'a': 1}");
        processor.process_closing_bracket(&'}');

        // After it notices that the line is complete, it prints the line
        // and clears the `jsonl_string`.
        assert_eq!(processor.jsonl_string.to_string(), String::from(""));
        assert_eq!(processor.bracket_stack.stack, vec!['[']);
    }

    #[test]
    fn test_process_other_char_pushes_char_to_jsonl_string() {
        let mut processor = Processor::new();
        processor.process_other_char(&'a');
        assert_eq!(processor.jsonl_string.to_string(), String::from("a"));
        assert_eq!(processor.bracket_stack.len(), 0);
    }

    #[test]
    fn test_should_print_true_if_bracket_stack_len_1() {
        let mut processor = Processor::new();
        processor.bracket_stack.push(&'[');
        assert_eq!(processor.should_print(), true);
    }

    #[test]
    fn test_should_print_false_if_bracket_stack_len_not_1() {
        let mut processor = Processor::new();
        processor.bracket_stack.push(&'[');
        processor.bracket_stack.push(&'{');
        assert_eq!(processor.should_print(), false);
    }

    #[test]
    fn test_process_char_flow_with_curly_inner_bracket() {
        let mut processor = Processor::new();
        processor.bracket_stack.push(&'[');

        // {
        processor.process_char(&'{');
        assert_eq!(processor.jsonl_string.to_string(), String::from("{"));
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{']);

        // {a
        processor.process_char(&'a');
        assert_eq!(processor.jsonl_string.to_string(), String::from("{a"));
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{']);

        // {a:
        processor.process_char(&':');
        assert_eq!(processor.jsonl_string.to_string(), String::from("{a:"));
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{']);

        // {a:{
        processor.process_char(&'{');
        assert_eq!(processor.jsonl_string.to_string(), String::from("{a:{"));
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{', '{']);

        // {a:{b
        processor.process_char(&'b');
        assert_eq!(processor.jsonl_string.to_string(), String::from("{a:{b"));

        // {a:{b:
        processor.process_char(&':');
        assert_eq!(processor.jsonl_string.to_string(), String::from("{a:{b:"));

        // {a:{b:1
        processor.process_char(&'1');
        assert_eq!(processor.jsonl_string.to_string(), String::from("{a:{b:1"));

        // {a:{b:1}
        processor.process_char(&'}');
        assert_eq!(processor.jsonl_string.to_string(), String::from("{a:{b:1}"));
        assert_eq!(processor.bracket_stack.stack, vec!['[', '{']);

        // {a:{b:1}}
        processor.process_char(&'}');
        assert_eq!(processor.jsonl_string.to_string(), String::from(""));
        assert_eq!(processor.bracket_stack.stack, vec!['[']);
    }

    #[test]
    fn test_process_char_flow_with_square_inner_bracket() {
        let mut processor = Processor::new();
        processor.bracket_stack.push(&'[');

        // [
        processor.process_char(&'[');
        assert_eq!(processor.jsonl_string.to_string(), String::from("["));
        assert_eq!(processor.bracket_stack.stack, vec!['[', '[']);

        // [a
        processor.process_char(&'a');
        assert_eq!(processor.jsonl_string.to_string(), String::from("[a"));
        assert_eq!(processor.bracket_stack.stack, vec!['[', '[']);

        // [a:
        processor.process_char(&':');
        assert_eq!(processor.jsonl_string.to_string(), String::from("[a:"));
        assert_eq!(processor.bracket_stack.stack, vec!['[', '[']);

        // [a:[
        processor.process_char(&'[');
        assert_eq!(processor.jsonl_string.to_string(), String::from("[a:["));
        assert_eq!(processor.bracket_stack.stack, vec!['[', '[', '[']);

        // [a:[b
        processor.process_char(&'b');
        assert_eq!(processor.jsonl_string.to_string(), String::from("[a:[b"));

        // [a:[b:
        processor.process_char(&':');
        assert_eq!(processor.jsonl_string.to_string(), String::from("[a:[b:"));

        // [a:[b:1
        processor.process_char(&'1');
        assert_eq!(processor.jsonl_string.to_string(), String::from("[a:[b:1"));

        // [a:[b:1]
        processor.process_char(&']');
        assert_eq!(processor.jsonl_string.to_string(), String::from("[a:[b:1]"));
        assert_eq!(processor.bracket_stack.stack, vec!['[', '[']);

        // [a:[b:1]]
        processor.process_char(&']');
        assert_eq!(processor.jsonl_string.to_string(), String::from(""));
        assert_eq!(processor.bracket_stack.stack, vec!['[']);
    }
}
