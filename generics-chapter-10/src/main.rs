/* Chapter 10: Generic Types, Traits, and Lifetimes
A generic is a stand in for a concrete type or property.
They can be used to reduce code duplication.

Traits are ways to define behavior in a generic way.
One can combine traits with generic types to constrain a generic
type to accept only those types that have a particular behavior,
as opposed to just any type.

Lifetimes: a variety of generics that give the compiler enough
information about how references relate to each other.
Lifetimes allow the developer to give the compiler enough information
about borrowed values so that it can ensure references will be
valid in more situations than it could without out help.

If generics start to require more than two types, e.g., <T, U>,
then it could indicate that the code needs to be restructured into
smaller pieces.

Generics don't make the program run any slower that it would with
concrete types.  Rust accomlishes this through monomophization, the
process of turning generic code into specific code.
*/

struct PointHomogeneous<T> {
    x: T,
    y: T,
}

// Define methods with a trait too:
impl<T> PointHomogeneous<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

// Instances of PointHomogeneous<T> where T is not of type
// f32 will not have this method defined.
impl PointHomogeneous<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point<T, U> {
    x: T,
    y: U,
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// We restrict to types that implement te PartialOrd trait.
// Traits will be discussed in the next section.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest character is {}", result);
    let result = largest(&char_list);
    println!("The largest character is {}", result);

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    let p = PointHomogeneous { x: 5, y: 10 };
    println!("p.get_x() = {}", p.get_x());
}
