use std::fs::File;
use std::io::{self, Read};

/// Custom Error Enum
/// This enum represents all possible errors that can occur in our program.
/// 1. IOError: Catches any errors during the file I/O (opening and reading).
/// 2. ContentError: Catches errors when the file content does not have "Ferris".
#[derive(Debug)]
enum FileError {
    IOError(io::Error),
    ContentError(String),
}

// Error conversion trait
// Implement the `From` trait to automatically convert a standard `io::Error`
// into our custom `FileError::IOError`.  This allows us to use the `?` operator
// on functions that return `Result<T, E>`.
impl From<io::Error> for FileError {
    fn from(error: io::Error) -> Self {
        FileError::IOError(error)
    }
}

/// Opens a file and returns the file handle.
/// 
/// Note: This function only returns a standard `io::Error`, but because we implement
/// `From<io::Error>` for `FileError`, we can use the `?` operator on it in
/// `read_file_content`.
///
/// # Arguments
/// * `file_path` - The name of the file to open.
fn open_file(file_path: &str) -> Result<File, io::Error> {
    println!("-> Attempting to open file: {}", file_path);
    // std::fs::File::open returns Result<File, io:Error>
    File::open(file_path)

}

/// Reads the entire content of the file into a String and validates the content.
/// 
/// The function returns our custom `Result<String, FileError>`.
/// 
/// # Arguments
/// * `file_path` - The name of the file to read.
fn read_file_content(file_path: &str) -> Result<String, FileError> {
    // Open the file, propagate io:Error using the `?` operator.
    // The `?` operator calls the `from` conversion for us, turning the
    // `io::Error` into `FileError::IOError`.
    let mut file = open_file(file_path)?;

    // Read content, propagate io::Error using the `?` operator.
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("-> File successfully read ({} bytes).", contents.len());

    // Content validation: check for "Ferris"
    if contents.contains("Ferris") {
        // Success: return the content wrapped in Ok.
        println!("-> Content check passed: 'Ferris' found.");
        Ok(contents)
    } else {
        // Failure: return our custom error variant wrapped in Err.
        println!("-> Content check failed: 'Ferris' not found.");
        Err(FileError::ContentError(format!(
            "The file '{}' did not contain the required keyword 'Ferris'.",
            file_path
        )))
    }
}


fn main() {
    let file_name = "rust.txt";

    match read_file_content(file_name) {
        // Success path
        Ok(content) => {
            println!("\nSuccess: File processed!");
            println!("File content snippit:\n\"{}\"...", &content.chars().take(50).collect::<String>());
        }
        // Error path: The calling function (main) handles the propagation error.
        Err(e) => {
            println!("\nError: Failed to process the file.");
            // Match the specific error variant to provide a tailored error handlings.
            match e {
                FileError::IOError(err) => {
                    eprintln!("I/O Error: Could not open or read the file '{}'", file_name);
                    eprintln!(" Details: {}", err);
                    // Suggest creating the file if it's a 'NotFound' error.
                    if err.kind() == io::ErrorKind::NotFound {
                        println!("  Hint: Please create a file named '{}' in the program directory.", file_name)
                    }
                }
                FileError::ContentError(msg) => {
                    eprintln!("Content error: {}", msg);
                    eprintln!("  Hint: Ensure the file contains the string 'Ferris'.");
                }
            }
        }
    }

    println!("\n--- Program finished ---");
    // Note: For a real test, you would need to create a `rust.txt` file with
    // content (e.g., "Hello, Ferris!") to see the success path.

}

