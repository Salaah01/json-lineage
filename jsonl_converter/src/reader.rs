//! This module contains the `ByteIterator` struct, which is used to iterate
//! over the bytes of a file. This allows us to read a file byte by byte,
//! instead of reading the entire file into memory at once.

use std::{
    fs::File,
    io::{self, BufReader, Read},
};

/// This struct is used to iterate over the bytes of a file.
pub struct ByteIterator {
    reader: BufReader<File>,
}

impl ByteIterator {
    /// Creates a new `ByteIterator` from a file.
    pub fn new(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        Ok(Self { reader })
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.next().unwrap().unwrap().chars().next()
    }
}

impl Iterator for ByteIterator {
    type Item = io::Result<String>;

    /// Returns the next byte of the file.
    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0; 1];
        match self.reader.read_exact(&mut buffer) {
            Ok(_) => Some(Ok(String::from_utf8_lossy(&buffer).into_owned())),
            Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(error) => Some(Err(error)),
        }
    }
}

/// Verifies that the first character of the file is a '['.
pub fn verify_first_char(first_char: &char) {
    if first_char != &'[' {
        panic!(
            "The first character of the file must be a '[', not a '{}'.",
            first_char
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_iterator_new_instance_accepts_valid_filename() {
        let bytes_iter = ByteIterator::new("src/reader.rs");
        assert!(bytes_iter.is_ok());
    }

    #[test]
    #[should_panic]
    fn test_bytes_iterator_new_instance_panics_on_invalid_filename() {
        let bytes_iter = ByteIterator::new("bad_filename");
        assert!(bytes_iter.is_ok());
    }

    #[test]
    fn test_can_iterate_over_bytes() {
        let bytes_iter = ByteIterator::new("src/reader.rs").unwrap();
        let mut bytes = String::new();

        for byte in bytes_iter {
            bytes.push_str(&byte.unwrap());
        }

        assert_eq!(bytes, include_str!("reader.rs"));
    }

    #[test]
    fn test_verify_first_char_passes() {
        verify_first_char(&'[');
    }

    #[test]
    #[should_panic]
    fn test_verify_first_char_panics_on_invalid_first_char() {
        verify_first_char(&'a');
    }
}
