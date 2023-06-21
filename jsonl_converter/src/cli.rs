//! Contains CLI related code.

use std::env;

/// Returns the filepath from the command line arguments assuming that the
/// filepath is the first argument.
///
/// # Returns
///
/// * The filepath from the command line arguments.
/// 
/// # Panics
/// 
/// * If the filepath is not provided.
pub fn get_filepath() -> String {
    let args: Vec<String> = env::args().collect();
    args[1].to_owned()
}
