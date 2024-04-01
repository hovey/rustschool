/* Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

* Crate
  * The smallest amount of code that the Rust compiler considers at a time
  * Takes a binary form or a library form
  * Binary crates
    * Has a main that defines what happens when the executable runs
    * All crates we have created so far have been binary crates
  * Library crates
    * Don't compile into an executable, instead they define functionality to
      beshared with multiple projects.
  * Crate root
    * source file that the Rust compiler starts from and makes up the root
      of the crate.
    * Cargo's convention is the src/main.rs as the crate root or to use
      src/lib.rs if it is a library crate
  * Package
    * Bundle of one or more crates that provide a set of functionality.
    * Contains a cargo.toml that describes how to build the crates.
*/

fn main() {
    println!("Hello, world!");
}
