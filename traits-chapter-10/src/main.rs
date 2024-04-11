/* Chapter 10.2: Traits: Defining Shared Behavior
A trait defines functionality that can be shared across types.

Traits are similar to interfaces in othr languages, but with some
differences.

Trait bounds specify that a generic type can be any type that has a certain behavior.

A trait can have multiple methods in its body.  The method signatures
are lsited one per line and each line ends in a semicolon.

The trait and the type must be part of the crate, a restriction
called the orphan rule, a property called coherence.  Otherwise,
two crates could implement the same trait for the same type, and
Rust wouldn't know which implementation to use.
*/

pub mod aggregator;

use aggregator::{Summary, Tweet};

fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
