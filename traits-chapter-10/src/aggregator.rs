// pub mod aggregator; <- nope, not here, put in main.rs

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// A Summary trait that consists of behavior provided by the
// summarize method.
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// A default implementation, as the trait is implemented on a
// particular type, we can keep the default implementation or
// override each method's default behavior.
pub trait Summary {
    // default implementation
    // Default implementations can call other methods in the same
    // trait, even if those other methods don't have a default
    // implementation.
    // It is not possible to call the default implementation
    // from an overriding implementation of that same method.
    fn summarize(&self) -> String {
        // String::from("(Read more...)")
        format!("(Read more from {}...)", self.summarize_author())
    }

    // a method without a default implementation, types with this
    // trait must provide their own implementation
    fn summarize_author(&self) -> String;
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// alternatively to use the default implementation of the
// summarize method
impl Summary for NewsArticle {
    // uses the default implemention of the summarize method

    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
}

// overwrites the default implementation
impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Instead of a concrete type for the item parameter, we specify
// the impl keyword and the trait name.  This parameter accepts
// any type that implements the specified trait.  Callers of notify
// can pass in any instance of NewsArticle or Tweet.  Calling code
// that calls notify with any other type, e.g., String or i32, won't
// compile because those types dont' implement the Summary trait.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The impl Trait syntax works for straightforward cases, but is
// actually syntax sugar for a longer form known as trait bound,
// which has the form:
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
// The impl Trait syntax is more convenient and more concise,
// while the fully trait bound synax can express more complexity.
// For exmaple, w can have two parameters that implement Summary:
pub fn notify_2(item1: &impl Summary, item2: &impl Summary) {}

// If we want to force both parameters to have the same type:
pub fn notify_3<T: Summary>(item1: &T, item2: &T) {}

// Multiple trait bounds can be specified with the + syntax.
// Using too many trait bounds can create functions that are
// difficult to read, with lots of trait bound information between
// the function's name and the parameter list.  So, rust has
// an alternate synax for specifying trait bounds inside a where
// clause.  So instead of this:
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
//
// fn some_function_improved<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

// We can also use the impl Trait syntax in the return position
// to return a value of some type that implements as trait:
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course a horse"),
        reply: false,
        retweet: false,
    }
}
// The ability to specify a return type only by the trait is useful
// in the context of closures and iterators, covered in Chapter 13.
// Closures and iterators create types that only the compiler knows
// or types that are very long to specify.

// The impl Trait return works only for a single concrete return
// type,multiple return types are not allowed.
// For example, returning eithr a NewsArticle or a Tweet is not
// allowed due to now the Rust compiler is implemented.
// So, this function below will not compile:

// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Trait bounds can be used to conditionally implement methods.
// Need to do more examples here.

// Blanket implementation occur when we conditionally implement
// a triat for any type that implements another trait.
// Blanket implementations appear in the documentation for the
// trait in the "Implementators" section.

// In dynamically typed languages, a runtime error occurs if we call
// a method on a type that doesn't implement that method.
// In contrast, Rust moves such errors to compile time, eliminating
// bugs before they can occur.  Also, we don't have to write code
// that checks for behavior at runtime because we have already
// done the check at compile time.  Doing this also improves
// performance without having to gie up the flexibility of generics.
