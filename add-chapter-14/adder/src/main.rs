use add_one;
use add_two;

fn main() {
    let num = 10;
    println!("Hello world! {num} plus one is {}!", add_one::add_one(num));
    println!("And again, hello world! {num} plus two is {}!", add_two::add_two(num));
}
