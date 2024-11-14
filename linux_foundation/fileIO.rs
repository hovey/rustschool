// Annotated Basic Rust Example: File I/O and Stdout

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    // Open the input file for reading using BufReader
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Read lines from the input file and process them
    for line in reader.lines() {
        let input_line = line?;
        let number: i32 = input_line.trim().parse().expect("Invalid number in file");

        // Perform a simple operation (multiply by 2 in this case)
        let result = number * 2;

        // Print the result to stdout
        println!("Input: {}, Output: {}", number, result);

        // Open the output file for writing
        let mut output_file = File::create("output.txt")?;

        // Write the result to the output file
        writeln!(output_file, "{}", result)?;
    }

    Ok(())
}


