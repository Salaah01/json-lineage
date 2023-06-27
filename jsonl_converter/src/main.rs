extern crate jsonl_converter;

use jsonl_converter::cli::get_filepath;
use jsonl_converter::processors::byte_processor::ByteProcessor;
use jsonl_converter::processors::line_processor::LineProcessor;
use jsonl_converter::readers::byte_iter::ByteIterator;
use jsonl_converter::readers::line_iter::LineIterator;
use jsonl_converter::readers::utils::verify_first_char;

fn main() {
    line_iter();
}

fn bytes_iter() {
    let mut bytes_iter = ByteIterator::new(&get_filepath()).unwrap();
    let first_char = bytes_iter.next_char().unwrap();
    verify_first_char(&first_char);

    let mut processor = ByteProcessor::new();
    processor.bracket_stack.push(&first_char);

    for byte in bytes_iter {
        let byte = byte.unwrap().to_owned().chars().next().unwrap();
        processor.process_char(&byte);
    }
}

fn line_iter() {
    let mut line_iter = LineIterator::new(&get_filepath()).unwrap();
    let first_line = line_iter.next_line().unwrap();
    let first_char = first_line.chars().next().unwrap();
    verify_first_char(&first_char);

    let mut processor = LineProcessor::new();
    processor.bracket_stack.push(&first_char);

    for line in line_iter {
        processor.process_line(&line);
    }
}
