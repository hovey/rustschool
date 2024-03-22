/*
Chapter 6
Enums and Pattern Matching

* define and use enumerations, the possible variants of an item
* The Option enum, a value that can be either something or nothing,
  defined in the standard library
* 'match' expression makes it easy to run different code per enums
* the 'if let' construct is a concise idiom to handle enums, where
  one pattern is matched but not the others.  'if let' is a non-
  exhaustive alternative to the exhaustive 'match'.
  So, 'if let' is syntax sugar for a 'match' statement that runs
  code when the value matches one pattern and then ignores all
  the other possible matches.

Any type of data can be put into enums and into each enum variant:
strings, numeric types, structs, or other enums.

*/

enum IpAddrKind {
    V4,
    V6,
}

// Strategy 1: Enum inside of a struct (less concise than Stategy 2).
struct IpAddrStrategy1 {
    kind: IpAddrKind,
    address: String,
}

// Strategy 2: Put enum data from IpAddrKind directly into each enum
// variant (more concise than Strategy 1).
enum IpAddrStrategy2 {
    V4(String),
    V6(String),
}

// Strategy 3: Enum within an enum can have heterogeneous data;
// whereas an enum inside of the struct, the struct is limited
// to homogeneous data.
enum IpAddrStrategy3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Strategy B1: Different structs, each of its own type.
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct

struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Strategy B2: different type variants inside of one enum.
// Now, a function can operate on all kinds of messages since it
// can operate on the Message enum, which is a single type.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Similar to structs, enums can have methods with the impl command.
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// enum Option<T> {
//     None,
//     Some(T),
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}.", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// in 'match', the 'other' is the catch-all
// match dice_roll {
//     3 => add_fancy_hat(),
//     7 => remove_fancy_hat(),
//     other => move_player(other),
// }
// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}

// in cases where we don't want to use the value in the catch-all
// use the '_' special pattern
// let dice_roll = 9;
// match dice_roll {
//     3 => add_fancy_hat(),
//     7 => remove_fancy_hat(),
//     _ => reroll(),  // or nothing happens, use _ => (),
// }
// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn reroll() {}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Strategy 1: Enum inside of a struct (less concise
    // than Stategy 2).
    let home = IpAddrStrategy1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStrategy1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    // Strategy 2: Put enum data from IpAddrKind directly into
    // each enum variant.
    let home_2 = IpAddrStrategy2::V4(String::from("127.0.0.1"));

    let loopback_2 = IpAddrStrategy2::V6(String::from("::1"));

    // Strategy 3: Enum with heterogeneous data inside an enum
    let home_3 = IpAddrStrategy3::V4(127, 0, 0, 0);
    let loopback_3 = IpAddrStrategy3::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
