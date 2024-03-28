/*
Implementing an Object-Oriented Design Pattern: State Pattern
Chapter 17.3

Define a set of states that a value can have.
States are represented by a set of state objects, and the value's
behavior changes based on its state.

We will have a blog post struct that has a field to hold its state,
which will be a state object from the set of "draft", "review", or
"published".

Rust will use structs and traits instead of objects and inheritance.
Each state object is responsible for its own behavior and for
knowing when it should change into another state.  The value that
holds a state object knows nothing about the different behavior of
the states, nor does it know when to transition between states.
*/

use blog::Post;

fn main() {
    println!("Hello, world!");

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());
}
