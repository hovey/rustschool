fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's values moves into the function, and so
                        // is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would normally move into the function,
                   // but i32 is Copy, so it's ok to still use x afterward

    // println!("Value of s: {s}"); // will not compile

    println!("Again, the value of x: {x}"); // will compile
} // Here, x does out of scope, then s.  Because s's values has moved, nothing
  // special happends here.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called.  The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer is out of scope.  Nothing special happens here.
