use crate::{
    brackets::{is_closing_bracket, is_opening_bracket, BracketStack},
    json_object::JSONLString,
};

pub struct Processor {
    bracket_stack: BracketStack,
    jsonl_string: JSONLString,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            bracket_stack: BracketStack::new(),
            jsonl_string: JSONLString::new(),
        }
    }

    pub fn push_bracket(&mut self, byte: &char) {
        self.bracket_stack.push(&byte);
    }

    pub fn process_char(&mut self, byte: &char) {
        match byte {
            b if is_opening_bracket(&b) => self.process_opening_bracket(b),
            b if is_closing_bracket(&b) => self.process_closing_bracket(b),
            _ => self.process_other_char(byte),
        }
    }

    fn process_opening_bracket(&mut self, byte: &char) {
        self.bracket_stack.push(&byte);
        self.jsonl_string.push_char(&byte);
    }

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

    fn process_other_char(&mut self, byte: &char) {
        self.jsonl_string.push_char(&byte);
    }

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
