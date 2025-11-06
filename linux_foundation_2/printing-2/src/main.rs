#[derive(Debug)]
struct Course {
    name: String,
    length: u8,
    trainer: String,
}



fn main() {
    let fname = "Linus";
    let lname = "Torvalds";

    println!("{0} {1}", fname, lname);
    println!("{fname} {lname}");

    // Exercise 3.3
    let numbers = [1, 2, 3, 4, 5];
    // Use debug format specifier
    println!("Numbers: {:?}", numbers);

    let person = ("Alice", 30, "Engineer");
    println!("Person: {:?}", person);  // prints inline
    println!("Person: {:#?}", person);  // prints with return characters

    // Exercise 3.4
    let x = 5;
    let y = 10;
    dbg!(x);

    let x = x + y;
    dbg!(x);

    // Derive expression
    let cc = Course {
        trainer: "Kooberstein".to_string(),
        length: 4,
        name: "Rusty Realms".to_string(),
    };
    // println!("{:?}", cc); // prints as one line
    println!("{:#?}", cc); // prints with newlines

    // let s = "Learning Rust"

    // To avoid the clippy warnings for dead code, we make sure to call each
    // part of the struct directly.
    println!("trainer: {}", cc.trainer);
    println!("length: {}", cc.length);
    println!("name: {}", cc.name);

}
