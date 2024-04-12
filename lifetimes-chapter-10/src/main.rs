/* Chapter 10.3 Validating References with Lifetimes
Lifetimes ensure that references are valie as long as we need them
to be.

Annotating lifetimes is not a concept most other programming
languages have, so it may feel unfamilar at first.

The lifetime syntax is about connecting the lifetime of a return
value of a function to the lifetime of the function parameters.

&i32          // a reference
&'a i32       // a reference with an explicit lifetime
&'a mut i32   // a mutable reference with an explicit lifetime

 */

// So far, we have created structs that hold owned types.  But, we
// can also define structs to hold references, but now we also need
// to include a lifetime annotation on every reference in the
// struct's definition

use std::fmt::Display;

struct ImportantExcerpt<'a> {
    // This annotation means an instance of ImportantExcerpt can't
    // outlive the reference it holds in in its part field:
    part: &'a str,
}

// Methods on structs with lifetimes must use the same syntax
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Generic type parameters, trait bounds, and lifetimes all in one
// function:
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("Hello, world!");
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael.  Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // A static reference lives for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";
}
