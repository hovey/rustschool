/* In Rust, Cow stands for "Clone on Write." It is a smart pointer type provided
by the standard library in the std::borrow module.  The main purpose of Cow is
to allow for efficient handling of data that can be either borrowed or owned,
depending on the context in which is it used.

What is Cow for?

Cow is an enum that can be represented as either:
* A borrowed reference to data (like a slice or a string slice).
* An owned version of that data (like a String or a Vec).

The two variants of Cow are:
* Borrowed(&T): A reference to data that is not owned.
* Owned(T): An owned copy of the data.

Why use Cow
* Efficiency: If one only needs to read data and not modify it, one can use the
borrowed variant, which avoids unnecessary copying.
* Flexibility: If one needs to modify the data, Cow can automatically clone the
borrowed data into an owned version when one writes to it.  This means one only
pays the cost of cloning when one actually needs to change the data.

Key points:
* Lazy Cloning: Cow allows you to avoid cloning data until you actually need to
modify it. This can lead to performance improvements, especially when dealing
with large amounts of data.
* Use Cases: Cow is particularly useful in scenarios when you want to work with
both borrowed and owned data, such as when writing functions that can accept
either type without forcing a clone unless necessary.
* Common Types: Cow is often used with types like str, Vec, and other collections
where you might want to work with slices or references without incurring the
cost of copying the entire data structure unless you need to modify it.
*/

use std::borrow::Cow;

fn main() {
    // Example 1: string slice and String
    //
    // "Hello, world!" is a string slice and is of type &'static str, which is
    // a reference to ta string slice that is stored in the program's binary
    // and has a static lifetime.  A string slice is immutable.
    // let borrowed: Cow<str> = Cow::Borrowed("Hello, world!"); // Borrowed data
    // doesn't compile without the 'mut'
    let mut borrowed: Cow<str> = Cow::Borrowed("Hello, world!"); // Borrowed data
    // "Hello, Rust" is a mutable string of String type, which is an owned and
    // growable string.
    let owned: Cow<str> = Cow::Owned(String::from("Hello, Rust!")); // Owned data

    // Use the borrowed data
    println!("{}", borrowed); // Output: Hello, world!

    // Use the owned data
    println!("{}", owned); // Output: Hello, Rust!

    // Modify the borrowed data
    // modified does not need to be mutable, per the compiler
    // let mut modified = borrowed.to_mut(); // Convert to mutable reference
    let modified = borrowed.to_mut(); // Convert to mutable reference
    modified.push_str(" Modified");  // This will clone the data
    println!("{}", modified); // Output: Hello, world! Modified!

    // Example 2: slice and Vector
    //
    // &[i32] is a reference to a contiguous sequence of elements in an array
    // or vector.  It does not own the data it points to.  For example, if you
    // have a Vec<i32>, you can create a slice from it using a reference,
    // &vec[..], which would give you a slice of the entire vector.
    //
    // vec![1, 2, 3, 4, 5] creates a Vec<i32>, which is a growable and owned
    // vector of integers.  The vec! macro is a convenient way to create a
    // new Vec and initialize it with the specified elements.
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Create a borrowed slice from the vector
    // doesn't compile without the 'mut'
    // let borrowed: Cow<[i32]> = Cow::Borrowed(&vec[..]); // Borrow the entire vector as a slice
    let mut borrowed: Cow<[i32]> = Cow::Borrowed(&vec[..]); // Borrow the entire vector as a slice

    // Create an owned version of the slice
    let owned: Cow<[i32]> = Cow::Owned(vec![6, 7, 8]); // Create a new owned vector

    // Print the borrowed slice
    println!("Borrowed: {:?}", borrowed); // Output: Borrowed: [1, 2, 3, 4, 5]

    // Print the owned slice
    println!("Owned: {:?}", owned); // Output: Owned: [6, 7, 8]

    // Modify the borrowed data
    // modified does not need to be mutable, per the compiler
    // let mut modified = borrowed.to_mut(); // Convert to mutable reference
    let modified = borrowed.to_mut(); // Convert to mutable reference
    modified[0] = 10; // This will clone the data since it's borrowed
    println!("Modified Borrowed: {:?}", modified); // Output: Modified Borrowed: [10, 2, 3, 4, 5]
}
