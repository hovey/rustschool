// Section 5-3: Methods are similar to functions because they start
// with the fn keyword and a name, and can have parameters and a
// return value.  Unlike functions, methods are defined in the
// context of a struct (or enum or trait), and their first parameter
// is always self, which represents the instance of the struct
// the method is called on.

// Methods can take ownership of self, borrow self immutably (as
// done below in this example), or borro self mutably.

// Associated Functions that aren't methods are often used for
// constructors that will return a new instance of a struct.

// Listing 5-1: A User struct definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple struct, name for the struct, but not for the fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Structs without fields, unit-lik structs
// no need for curly braces
struct AlwaysEqual;

// Listing 5-10: (partial)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// The Method syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Function
    fn square(size: u32) -> Self {
        // note absence of &self in the square signature
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Hello, world!");

    // Listing 5-2: Creating an instance of the User struct
    let mut user1 = User {
        active: true,
        username: String::from("some_user_name"),
        email: String::from("some_user@gmail.com"),
        sign_in_count: 1,
    };

    // Listing 5-7: New instance with the struct update syntax
    let user2 = User {
        email: String::from("the-second-user@gmail.com"),
        ..user1
    };

    // Listing 5-3: Update the value in the email field
    user1.email = String::from("another_email@example.com");

    // use of two tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // use of a unit-like struct
    let subject = AlwaysEqual;

    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30, 50);
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        // area(rect1)
        area(&rect2)
    );

    println!(
        "The area of the rectangle, via a method call, is {} square pixels.",
        rect2.area()
    );

    // used for debugging
    println!("debugging rect2 {:?}", rect2);
    println!("also debugging rect2 {:#?}", rect2);

    // Method Syntax
    // rust/html/book/ch05-03-method-syntax.html
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Use the Associated Function
    let sq1 = Rectangle::square(60);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold sq1? {}", rect1.can_hold(&sq1));
}

// Listing 5-8: Area of rectangle calculation
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Listing 5-9: Specify width and height with a tuple
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Listing 5-10: Defining the function on the Rectangle struct
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Listing 5-4: Factory that returns a new User instance
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Listing 5-5: Use of field init shorthand with username and email parameters
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
