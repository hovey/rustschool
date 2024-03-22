/* Characteristics of Object-Oriented Languages

* With structs and enums as object data, and with impl as object
  methods, one can say that Rust is object-oriented
* Rust encapulation with 'pub'.
* Rust does not have inheritance.  "There is no way to define a
  struct that nherits from the parent's struct's fields and
  method implementations without using a macro."
  * But, Traits, when overriden from the default behavior, can act
    as in a similar way as inheritance.
  * For Polymorphism, wherein a child type can be used in the same
    places as a parent type, Rust uses generics to abstract over
    different possible types and trait bounds to impose constraints
    on what what those types must provide.  This is sometimes called
    'bounded polymorphism'.
  * Inheritance has recently fallen out of favor because it often
    creates sharing of more code than is necessary.
*/

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
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
}
