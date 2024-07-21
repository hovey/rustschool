//! This library provides utilities for command line tools.
//! Currently, it only provides a function to read a line from stdin.
//! # Examples:
//! ```
//! use cli_utils::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! The `read_stdin` function will panic with "Failed to read input line." if it
//! fails to read.


use std::io::{BufReader, BufRead};

/// This function reads a line from stdin and returns it as a `String`.
/// It will panic with "Failed to read input line." if it fails to read.
/// # Examples:
/// ```
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input line.");
    line.trim().to_string()
}
