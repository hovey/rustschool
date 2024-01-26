trait Describable {
    fn describe(&self) -> String {
        // default implementation
        "Uninplemented".to_string()
    }
}

struct Person {
    name: String,
    age: u32,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

fn print_description(item: &impl Describable) {
    println!("{}", item.describe());
}

fn print_description_destructive(item: impl Describable) {
    println!("{}", item.describe());
}

fn main() {
    let person = Person {
        name: "Adam".to_string(),
        age: 20,
    };

    // demonstrate borrowing by moving the print statements out of the
    // main

    // person gets destroyed after the print_description_destructive call
    print_description_destructive(person);

    // since person is destroyed above, there is no longer a person to pass to
    // print_description
    print_description(&person);
}
