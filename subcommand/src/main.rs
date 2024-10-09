//! This module show how two subcommands can be used.
//! 
//! Example
//! -------
//! 
//!  cargo build
//!  cargo run -- add --num1 5 --num2 --precision 3
//!  cargo run -- subtract --num1 5 --num2 3 --message hello
//!  cargo run -- -h
//!  cargo run -- add -h
//!  cargo run -- subtract -h
//! 
//! 
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "calculator")]
#[command(about = "A simple calculator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add two numbers
    Add {
        /// The first number
        #[arg(long)]
        num1: f64,
        /// The second number
        #[arg(long)]
        num2: f64,
        /// An optional precision for the result
        #[arg(long, default_value_t = 2)]
        precision: usize,
    },
    /// Subtract two numbers
    Subtract {
        /// The first number
        #[arg(long)]
        num1: f64,
        /// The second number
        #[arg(long)]
        num2: f64,
        /// An optional string to display
        #[arg(long)]
        message: Option<String>,
    },
}


fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { num1, num2, precision } => {
            let result = num1 + num2;
            println!("Result of addition: {:.precision$}", result);
        }
        Commands::Subtract { num1, num2, message } => {
            let result = num1 - num2;
            match message {
                Some(msg) => println!("{}: {}", msg, result),
                None => println!("Result of subtraction : {}", result),
            }
        }
    }
}
