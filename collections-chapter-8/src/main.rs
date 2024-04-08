use std::{hash::Hash, str::SplitTerminator};

fn main() {
    println!("Hello, world!");

    // Explicit annotation of vector type i32
    let v: Vec<i32> = Vec::new();

    // More often, Rust will infer the type of value to store.
    let v = vec![1, 2, 3]; // type i32 is inferred

    // updating a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    // reference a stored vector via index
    // References to a non-existent or out of index item will cause the
    // program to panic and crash.
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // reference a stored vector via the get method
    // Used when your code has logic to handle out of range requests, without
    // a panic and crash.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // iterating over values in a vector
    let v = vec![100, 32, 57];
    // immutable reference i to each element in a vector of i32
    for i in &v {
        println!("{i}");
    }

    // mutable reference for each item, mutable lets us make changes in place
    let mut v = vec![2, 4, 6];
    for i in &mut v {
        println!("The value before the mutable reference is {i}");
        *i += 50; // dereference operator * to get the value pointed to by i
        println!("The value after the mutable reference is {i}");
    }

    // using an enum to store multiple types
    // enums are useful in the case where the types are known ahead of compile
    // time.  For types not known ahead of compile time, the enum technique
    // will not work (instead, one will have to use a trait object, to be
    // discussed in Chapter 17.)
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // like any other struct, a vector is freed when it goes out of scope
    {
        let vv = vec![1, 2, 3, 4];
        // do stuff with vv
    } // <- vv goes out of scope and is freed here

    // Chapter 8.3: Storing UTF-8 Encoded Text with Strings
    // only the string slice 'str' is coded into the Rust core language
    // A str slice is a reference to some UTF-8 encoded string data
    // The 'String' type is in Rust's standard library (not its core language)
    // String is growable, mutable, and owned UTF-8 string type.

    let data = "initial contents";
    let s = data.to_string();

    // also works on literal directly
    let s = "literal initial contents".to_string();

    // also the 'from' command
    let s = String::from("still another way");

    // appending
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push takes a single characters (not a string)
    let mut s = String::from("lo");
    s.push('l'); // s now contains "lol"

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved and thus no longer used

    // For multiple strings, the '+' operator gets unwieldly
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // "tic-tac-toe"

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // so easier to use the format! macro
    let s = format!("{s1}-{s2}-{s3}"); // "tic-tac-toe"

    // cannot reference into a String
    let s1 = String::from("hello");
    // let h = s1[0]; // will not compile

    // best way to operate on pices of string
    for c in "hello".chars() {
        println!("{c}");
    }

    for b in "hello".bytes() {
        println!("{b}");
    }

    // useful String methods: 'contains' and 'replace'

    // Chapter 8.3: Storing keys with associated values in a Hash Map (dictionary)
    // Hash Map is also known as a hash, map, hash table, dictionary, associative array
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Like vectors, hash maps are homogeneous: all keys must have the same type, and
    // all values must have the same type.

    // The 'get' method returns an Option<&V);, returns the value or None.
    // The 'unwrap_or' sets the 'score' to zero if scores has no entry for the key.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // 10

    // iterate over has maps similar as with vectors
    for (key, value) in &scores {
        println!("{key}: {value}"); // prints in arbitrary order
    }

    // The Hash Map becomes the owner of key and value
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are now invalid since they are owned by map
    // However, if we were to insert references into the hash map, the values won't
    // be moved.

    // overwrite (update) values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrite 10 with 25

    // add a key and value only if a key is not present via the 'entry' method
    // The 'entry' method returns an enum called Entry
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // update a value based on an old value, example: word counter
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert returns a mutable reference (&mut V) to the value
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{:?}", map);

    // By default, HashMap uses a hashing function called SipHash that can provide
    // resistance to Denial of Service (DoS) attacks involving has tables.  This is
    // not the fastest hasing algorithm, but the trade-off comes for better security.
}
