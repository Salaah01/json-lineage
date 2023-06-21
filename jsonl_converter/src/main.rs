extern crate jsonl_converter;

use jsonl_converter::brackets::{is_closing_bracket, is_opening_bracket, BracketStack};
use jsonl_converter::json_object::JSONLString;
use jsonl_converter::reader::{verify_first_char, ByteIterator};
use jsonl_converter::cli::get_filepath;

fn main() {
    let mut bytes_iter = ByteIterator::new(&get_filepath()).unwrap();
    let first_char = bytes_iter.next_char().unwrap();

    verify_first_char(&first_char);

    let mut bracket_stack: BracketStack = BracketStack::new();
    bracket_stack.push(&first_char);

    let mut jsonl_string = JSONLString::new();

    for byte in bytes_iter {
        let byte = byte.unwrap().to_owned().chars().next().unwrap();

        match byte {
            b if is_opening_bracket(&b) => {
                bracket_stack.push(&b);
                jsonl_string.push_char(&b);
            }
            b if is_closing_bracket(&b) => {
                bracket_stack.pop_pair(&b).unwrap();

                if bracket_stack.len() == 1 {
                    jsonl_string.push_char(&b);
                    println!("{}", jsonl_string);
                    jsonl_string.clear();
                } else {
                    jsonl_string.push_char(&b);
                }
            }
            _ => {
                jsonl_string.push_char(&byte);
            }
        }
    }
}
