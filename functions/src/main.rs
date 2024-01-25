fn main() {
    println!("Hello, world!");

    another_function(7);

    print_labeled_measurement(6, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(5);
    println!("The value of 'x + 1' is {y}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
