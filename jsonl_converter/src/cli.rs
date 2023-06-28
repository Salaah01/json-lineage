//! Contains CLI related code.

use std::env;

/// Returns the filepath from the command line arguments assuming that the
/// filepath is the first argument.
///
/// Optionally, a `--messy` flag can be provided to indicate that the JSONL
/// file is not well formed. This is useful if the JSONL file contains
/// multiple JSON objects on a single line.
///
/// # Returns
///
/// * The filepath from the command line arguments.
/// * A boolean indicating whether the JSONL file is not well formed.
///
/// # Panics
///
/// * If the filepath is not provided.
pub fn parse_args() -> (String, bool) {
    let mut args = env::args_os();
    args.next(); // Skip the program name.

    let filepath = args.next().expect("No filepath provided.");
    let is_messy = if let Some(arg) = args.next() {
        arg == "--messy"
    } else {
        false
    };

    (filepath.into_string().unwrap(), is_messy)
}
