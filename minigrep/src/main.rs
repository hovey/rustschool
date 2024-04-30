use std::env;
use std::process;

// 2024-04-29: Stopped today after completing section 12.4.
// Start next on section 12.5.

use minigrep::Config;

fn main() {
    // println!("Hello, world!");
    // We explicitly annotate the type of args as a vector of
    // String values, viz., Vec<String>.
    // We rargely need to annotate in Rust, but do
    // it here because Rust isn't able to infer the kind of
    // collection without the annotation.
    let args: Vec<String> = env::args().collect();
    // print the args via the dbg! macro
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Search word: {}", config.query);
    // println!("In file: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
