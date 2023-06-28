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
    let args: Vec<String> = env::args().collect();
    let filepath = args[1].to_owned();
    let mut is_messy = false;

    if args.len() > 2 {
        is_messy = args[2] == "--messy";
    }

    (filepath, is_messy)
}
