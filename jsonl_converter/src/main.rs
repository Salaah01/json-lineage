extern crate jsonl_converter;

use jsonl_converter::cli::get_filepath;
use jsonl_converter::processors::byte_processor::ByteProcessor;
use jsonl_converter::readers::byte_iter::ByteIterator;
use jsonl_converter::readers::utils::verify_first_char;

fn main() {
    let mut bytes_iter = ByteIterator::new(&get_filepath()).unwrap();
    let first_char = bytes_iter.next_char().unwrap();

    verify_first_char(&first_char);

    let mut processor = ByteProcessor::new();
    processor.push_bracket(&first_char);

    for byte in bytes_iter {
        let byte = byte.unwrap().to_owned().chars().next().unwrap();
        processor.process_char(&byte);
    }
}
