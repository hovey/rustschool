fn sqrt(number: f64) -> Result<f64, String> {
    if number < 0.0 {
        Err(format!("Cannot calculate square root of negative number {}", number))
    } else {
        Ok(number.sqrt())
    }
}

fn main() {
    let value: f64 = 64.0;
    match sqrt(value) {
        Ok(root) => println!("Success: The root is {}", root),
        Err(e) => println!("Error: {}", e),
    }

    let value: f64 = -64.0;
    match sqrt(value) {
        Ok(root) => println!("Success: The root is {}", root),
        Err(e) => println!("Error: {}", e),
    }

}
