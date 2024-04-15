fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // String::from("Hello!")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        if value < 1 {
            // panic!(
            //     "Guess value must be between 1 and 100, inclusive, got {}.",
            //     value
            // )
            panic!("Guess values must be >= 1, got {}", value)
        } else if value > 100 {
            panic!("Guess values must be <= 100, got {}", value)
        }
        Guess { value } // wow! that is neat notation
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    //  fn it_works() {
    fn it_works() -> Result<(), String> {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // Writing tests so they return a Result<T, E> enables you to
    // use the question mark operator in the body of the tests,
    // which can be a convenient way to write tests that should
    // fail if any operation within them returns an Err variant.
    //
    // One cannot use the #[should_panic] annotation on tests that
    // use the Result<T, E>.
    //
    // To assert that an operation returns an Err variant, don't
    // use the question mark operator on the Result<T, E> value.
    // Instead, use assert!(value.is_err()).

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     // Test designed to fail to demonstrate test failure
    //     panic!("Make this test fail");
    // }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
