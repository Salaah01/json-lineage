//! This module contains the `LineIterator` struct, which is used to iterate
//! over the lines of a file. This allows us to read and process a file line by
//! line, instead of reading the entire file into memory at once.

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

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
    fn test_line_iter_new_instance_accepts_valid_filename() {
        let line_iter = LineIterator::new("src/readers/mod.rs");
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
}
