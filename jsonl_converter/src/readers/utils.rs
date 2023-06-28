///! This module contains utilities for the `readers` module.

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
/// use jsonl_converter::readers::utils::verify_first_char;
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

#[cfg(test)]
mod tests {
    use super::*;

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
