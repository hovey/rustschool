/*
Best Practices for Packages with a Binary and a Library

"We mentioned a package can contain both a src/main.rs binary crate root as well as a src/lib.rs library crate root, and both crates will have the package name by default. Typically, packages with this pattern of containing both a library and a binary crate will have just enough code in the binary crate to start an executable that calls code with the library crate. This lets other projects benefit from the most functionality that the package provides, because the library crate’s code can be shared."

"The module tree should be defined in src/lib.rs. Then, any public items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: it can only use the public API. This helps you design a good API; not only are you the author, you’re also a client!"

*/

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    // Structs are often useful without all of their fields being public, so
    // struct fields follow the general rule of everything being private
    // by default unless annotated with pub.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Enums aren't very useful unless their variants are public.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// The idiomatic way of using functions is to stop at the parent level,
// so that the parent is used below to specify the scope of where a function
// comes from.
// use crate::front_of_house::hosting;

// The re-exporting of hosting, with use of pub
// External code can now use the path restaurant::hosting::add_to_waitlist()
// pub use crate::front_of_house::hosting;

// Alternatively, when bringing in structs, enums, and other items with use,
// it is idiomatic to specify the full path, as here with HashMap (see
// book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths)

pub fn eat_at_restaurant() {
    /*
    "Our preference in general is to specify absolute paths because it's more
    likely we'll want to move code definitions and item calls independently of
    each other."
     */

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // use the use keyword used above
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind about what bread we would like
    meal.toast = String::from("Wheat");
    println!("I would like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
