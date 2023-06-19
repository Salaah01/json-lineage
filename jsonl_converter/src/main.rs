extern crate jsonl_converter;

use jsonl_converter::reader::ByteIterator;

fn main() {
    let bytes_iter = ByteIterator::new("src/reader.rs").unwrap();

    for byte in bytes_iter {
        print!("{}", byte.unwrap().to_owned());
    }
}
