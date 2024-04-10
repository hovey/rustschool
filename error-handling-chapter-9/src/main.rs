use std::error:Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

// 2024-04-10: Stop, to start at 9.3 To panic! or Not to panic!

// Listing 9-6: A function that returns errors to the calling
// code using match.  The function returns a type Result<T, E>,
// where generic type T has been filled with the concrete type
// String and generic type E has been filled with concrete type
// io::Error.
// The code that calls this function gets to handle either getting
// an Ok value that contains a username, or an Err value that
// contains an io::Error.  It's up to the calling code to decide
// what to do with those values, thus propagating the error.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("file_not_exist.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// A shortcut using the ? operator
fn read_username_from_file_2() -> Result<String, io::Error> {
    // The ? is placed after a Result value is defined.  If the
    // Result is an Ok, the value inside of the Ok will get returned,
    // and if the value is an Err, the Err will be returned from the
    // whole function as if we have used the return keyword so the
    // error gets propagated to the calling code.
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// An even shorter version:
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// And now, an even shorter (really!?) version:
fn read_username_from_file_4() -> Result<String, io::Error> {
    // Reading a file is a common operation, so the standard library
    // provjes a convenient fs::read_to_string function that opens
    // the file, creates a new String, reads the contents of the
    // file, and returns it.  This method, however, doesn't give us
    // the opportunity to explain all the error handling.
    fs::read_to_string("hello.txt")
}

/*
Note the ? operator can be used on an Option in a function that
returns an Option, or on a Result in a fuction  that returns a
Result, but these cannot be mixed.  The ? operator won't convert a
Result to an Option or vice versa; in those cases, you can use
methods like the ok method on a Result or the ok_or method on
Option to do the conversion explicitly.
*/
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

/*
The Box<dyn Error> is a trait object, to be discussed in Chapter 17,
Using Trait Objects that Allow for Values of Different Types.
Box<dyn Error> can be read as "any kind of error".
*/
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

fn main_old() {
    println!("Hello, world!");
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];
    let file_name = String::from("hello.txt");
    // let greeting_file_result = File::open("hello.txt");
    let greeting_file_result = File::open(&file_name);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // ErrorKind::NotFound => match File::create("hello.txt") {
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", error);
            }
        },
    };

    // In Chapter 13, we will learn about closures, which can be
    // more concise than using match when handling Result<T, E>
    // values.
    // The 'unwrap_or_else' can help to clean up huge nested
    // match expressions when dealing with errors.
    let greeting_file_cl = File::open(&file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&file_name).unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });

    // The 'unwrap' method is a shortcut method implemented just like
    // the match expression written above.  If the Result value is
    // of the Ok variant, unwrap will just return the value inside
    // of the Ok.  If the Result is the Err variant, unwrap will call
    // the panic! macro.  Here is an example:
    let greeting_file_3 = File::open(&file_name).unwrap();

    // Similarly, the expect method lets us choose the panic! error
    // message.  Using expect instead of unwrap can make tracking
    // down the source of panic easier.  Example:
    let error_msg = format!("{file_name} should be included");
    let greeting_file_4 = File::open(&file_name).expect(&error_msg);

    // In production code, most Rustaceans choose expect rather than
    // unwrap and give more context about why the operation is
    // expected to succeed.

    // Propagating Errors
    // Instead of handing an error, you can return the error to the
    // calling code, this is known as propagating the error, and
    // gives more control to the calling code.

    // Example, Listing 9-6.  A function that reads a username from
    // a file.  If the file doesn't exist or can't be read, the
    // function will return those errors to the code that called
    // the function.
}
