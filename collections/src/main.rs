fn main() {
    println!("Hello, world!");

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}.");

    // Option returns the element or None.  If the index is outside of the
    // vector, get returns None without panicking.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}."),
        None => println!("There is no third element."),
    }

    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("before {i}");
        *i += 50;
        println!("after {i}");
    }

    // Using the Enum to Store Multiple Types

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Strings

    let mut s = String::from("foo");
    s.push_str("bar"); // contains "foobar"

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Use format! macro for unwieldy strings
    // let s = s1 + "-" + &s2 + "-" + &s3; // tic-tac-toe, unwieldy syntax
    let s = format!("{s1}-{s2}-{s3}"); // better

    // The format! macro uses references, so the code above doesn't take ownership

    // Indexing into Strings
    // avoid indexing and slicing into Strings, because of Unicode
    // The best way to operate on pieces of strings is to iterate with chars()

    for c in "Зд".chars() {
        println!("{c}");
    }
    // produces
    // З
    // д

    // useful String and &str operations:
    // contains
    // replace

    // Hash maps, type HashMap<K, V>
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    let team_name = "Blue".to_string();
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // iteration
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // Yellow: 50
    // Blue: 10

    // Ownership
    // For owned values like String, the values will be moved and the hash map
    // will be the owner of those values:
    let field_name = "Favorite color".to_string();
    let field_value = "Blue".to_string();

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are now invalid

    // println!("field_name: {field_name}"); // does not compile

    // Overwriting a Value
    // If key, value exists, and then insert that same key with a different value,
    // the value associated with that key will be replaced

    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Blue".to_string(), 25); // 25 overwrites the 10

    // Adding a key and a value only if a key isn't present
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);

    scores.entry("Yellow".to_string()).or_insert(50);
    scores.entry("Blue".to_string()).or_insert(50);

    println!("{:?}", scores); // prints {"Yellow": 50, "Blue": 10}

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // prints: {"world": 2, "hello": 1, "wonderful": 1}

    // Hashing functions
    // HashMap uses a hashing function called SipHash, which is not the fastest, but
    // provides resistance to Denial of Service (DoS) attacks that involve hash tables.
}
