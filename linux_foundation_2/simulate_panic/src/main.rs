fn check_age(age: i32)
{
    if age < 0 {
        panic!("Age cannot be less than zero.")
    }
    println!("Age {} is valid.", age);
}


fn main() {
    let a1 = 0; // valid age
    let a2: i32 = -1; // invalid age

    // Should not panic
    check_age(a1);

    // Should panic
    check_age(a2);
}
