fn main() {
    // Chapter 3.5 Control Flow
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of the number is: {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut nn = 3;

    // Lift off, with a mediocre way of looping:
    while nn != 0 {
        println!("{nn}!");
        nn -= 1;
    }

    println!("LIFTOFF!!!");

    // Lift off, with a better way of looping:
    for qq in (1..4).rev() {
        println!("{qq}!");
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // An error-prone example:
    while index < 5 {
        // not like this:
        // println!("the value is: {a[index]}");
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // A better way:
    for element in a {
        println!("the value is: {element}");
    }
}
