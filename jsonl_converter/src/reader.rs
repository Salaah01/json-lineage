//! This module contains the `ByteIterator` struct, which is used to iterate
//! over the bytes of a file. This allows us to read a file byte by byte,
//! instead of reading the entire file into memory at once.

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

/// Verifies that the first character of the file is a '['.
///
/// # Arguments
///
/// * `first_char` - The first character of the file.
///
/// # Panics
///
/// * If the first character of the file is not a '['.
///
/// # Examples
///
/// ```
/// use jsonl_converter::reader::verify_first_char;
///
/// let first_char = '[';
/// verify_first_char(&first_char);
/// ```
pub fn verify_first_char(first_char: &char) {
    if first_char != &'[' {
        panic!(
            "The first character of the file must be a '[', not a '{}'.",
            first_char
        );
    }
}

/// This struct is used to iterate over the bytes of a file.
///
///
/// # Fields
///
/// * `reader` - A `BufReader` that reads the file.
pub struct ByteIterator {
    reader: BufReader<File>,
}

impl ByteIterator {
    /// Creates a new `ByteIterator` from a file. This is used to iterate over
    /// the bytes of a file.
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the file.
    ///
    /// # Errors
    ///
    /// * If the file cannot be opened.
    pub fn new(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        Ok(Self { reader })
    }

    /// Returns the next character of the file.
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

pub struct LineIterator {
    reader: BufReader<File>,
}

impl LineIterator {
    pub fn new(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        Ok(Self { reader })
    }

    /// Returns the next line of the file.
    pub fn next_line(&mut self) -> Option<String> {
        let mut buffer = String::new();
        match self.reader.read_line(&mut buffer) {
            Ok(s) => {
                if s == 0 {
                    return None;
                }
                Some(buffer)
            }
            Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(_) => None,
            _ => None,
        }
    }
}

impl Iterator for LineIterator {
    type Item = String;

    /// Returns the next line of the file.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_line()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_iter_new_instance_accepts_valid_filename() {
        let bytes_iter = ByteIterator::new("src/reader.rs");
        assert!(bytes_iter.is_ok());
    }

    #[test]
    #[should_panic]
    fn test_byte_iter_new_instance_panics_on_invalid_filename() {
        let bytes_iter = ByteIterator::new("bad_filename");
        assert!(bytes_iter.is_ok());
    }

    #[test]
    fn test_byte_iter_can_iterate_over_bytes() {
        let bytes_iter = ByteIterator::new("src/reader.rs").unwrap();
        let mut bytes = String::new();

        for byte in bytes_iter {
            bytes.push_str(&byte.unwrap());
        }

        assert_eq!(bytes, include_str!("reader.rs"));
    }

    #[test]
    fn test_line_iter_new_instance_accepts_valid_filename() {
        let line_iter = LineIterator::new("src/reader.rs");
        assert!(line_iter.is_ok());
    }

    #[test]
    #[should_panic]
    fn test_line_iter_new_instance_panics_on_invalid_filename() {
        let line_iter = LineIterator::new("bad_filename");
        assert!(line_iter.is_ok());
    }

    #[test]
    fn test_line_iter_can_iterate_over_lines() {
        let fp = "tests/line_iter_testcase.txt";
        let line_iter = LineIterator::new(fp).unwrap();
        let mut lines = String::new();

        for line in line_iter {
            println!("{}", line);
            lines.push_str(&line);
        }

        assert_eq!(
            lines,
            "This is line 1\n  This is line 2\nThis is line 3  \n"
        );
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
