fn main() {
    let numerator = 10;
    // let denominator = 0; // change to non-zero to see the division result
    let denominator = 3; // change to non-zero to see the division result

    match divide(numerator, denominator) {
        Some(result) => println!("The result of the division is: {}", result),
        None => println!("Error: Division by zero is not allowed."),
    }
}

fn divide(numerator: i32, denominator: i32) -> Option<f32> {
    match denominator {
        0 => None,
        _ => Some(numerator as f32 / denominator as f32),
    }
}
