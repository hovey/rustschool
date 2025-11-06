fn not_take_ownership(strings: &Vec<String>) {
    for item in strings {
        println!("The string is {}", item);
    }
}



fn main() {
    let ss: Vec<String> = vec![
        String::from("Hello World!"),
        String::from("foo bar"),
        String::from("Rust"),
    ];
    // not_take_ownership(ss);
    not_take_ownership(&ss);

    for item in ss {
        println!("The string is {}", item);
    }

}
