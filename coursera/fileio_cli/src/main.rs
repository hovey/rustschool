// Created ~/rustschool/coursera/file_io/test.txt with contents: Hello world!\nBig world.
// ~/rustschool/coursera/file_io/cargo run -- test.txt
// Reference: https://doc.rust-lang.org/rust-by-example/std_misc/arg.html
//
// Output:
// My path is target/debug/fileio_cli.
// I got 1 argument(s): ["test.txt"].
// Hello world!
// Big world.

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} argument(s): {:?}.", args.len() - 1, &args[1..]);


    // let file = File::open("non_existent_file.txt");
    let file = File::open(&args[1]);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}