// 1. Add a lifetime parameter to the struct definition
struct Person<'a> {
    name: &'a str, // 2. Apply the lifetime to the reference field.
}

// 3. Apply the same lifetime to the function signature to relate the input
// reference's lifetime to the output struct's lifetime.
fn make_person<'a>(name: &'a str) -> Person<'a> {
    Person { name }
}

fn main() {
    let name = "Alice";
    let alice = make_person(name);
    println!("Name: {}", alice.name);
}
