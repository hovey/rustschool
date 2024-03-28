/* Characteristics of Object-Oriented Languages (Chapter 17)

* With structs and enums as object data, and with impl as object
  methods, one can say that Rust is object-oriented
* Rust provides encapulation with 'pub' (public) functions.
* Rust does NOT have inheritance.  "There is no way to define a
  struct that inherits from the parent's struct's fields and
  method implementations without using a macro."
  * But, Traits, when overriden from the default behavior, can act
    as in a similar way as inheritance.
  * For Polymorphism, wherein a child type can be used in the same
    places as a parent type (or more generally, code that work
    with multiple types, not just inherited types), Rust uses
      * generics to abstract over different possible types, and
      * trait bounds to impose constraints
    on what what those types must provide.  This is sometimes called
    'bounded parametric polymorphism'.
  * Inheritance has recently fallen out of favor because
    * it often creates sharing of more code than is necessary, and
    * subclasses shouldn't always share all characters of a parent.
* To provide polymorphism, Rust uses trait objects instead
  of inheritance.
* Compared to a struct or an enum, a trait object, which combines
  data and behavior, is more like an object in classic OOP.  However,
  unlike traditional objects, trait objects cannot have data added.
* The purpose of a trait object is to allow abstraction across
  common behavior.
  * The concept of being concerned only with the messages a value
    responds to rather than the value's concrete type is similar to
    the concept of duck tying in dynamically typed languages.
* run doesn't need to know what the concrete type of each component
  is, and it doesn't do type checking to see if a component is a
  Button or a SelectBox.  By specifying Box<dyn Draw> as the type
  of the values in the components vector, we defined Screen to need
  values that can call the draw method.
* The advantage of using trait obejcts and Rust's type system to
  write code similar to duck typing is that it avoids client run
  time identification (RTTI).  We don't have to check if a value
  implements a particular method at runtime.  Nor do we worry
  about errors if a value doesn't implement a method but we call it
  anyway.  This is because Rust won't complile code on values that
  don't implement a trait that the trait object needs.
* With trait objects, Rust must use dynamic dispatch.  At runtime,
  Rust uses the pointers inside the trait object to know which method
  to call.  The lookup incurs a runtime cost that doesn't occur with
  static dispatch.
  * Dynamic dispatch prevents the compiler from inlining a method's
    code, which in turn, prevents some optimizations.
  * However, dynamic dispatch provides code flexibility, so it is
    a trade off for the decreased performance relative to static
    dispatch.
*/

pub struct AveragedCollection {
    list: Vec<i32>, // private
    average: f64,   // private
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Holds a vector of type Box<dyn Draw>, which is a trait
    // object and a stand-in for any type inside a Box that
    // implements the Draw trait.
    pub components: Vec<Box<dyn Draw>>,
}

// On the Screen struct, we define a method called run that will
// call the draw method on each of its components.
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("I am the draw method implementing the Draw trait for Button with width {}, height {}, and label{}.", self.width, self.height, self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("I am the draw method implementing the Draw trait for SelectBox with width {}, height {}, and options{:?}.", self.width, self.height, self.options);
    }
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    println!("Hello, world!");
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe");
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
