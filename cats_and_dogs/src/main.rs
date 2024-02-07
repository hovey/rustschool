// Output:
// My name is Sylvester, language is meow.  My grayscale is 128.
// My name is Fido, language is bark.  My RGB colors are 11, 22, 33.

struct Animal {
    name: String,
    language: String
}

struct Cat {
    grayscale: u32,
    animal: Animal
}

struct Dog {
    red: u32,
    green: u32,
    blue: u32,
    animal: Animal
}

trait Describable {
    fn describe(&self) -> String;
}

impl Describable for Cat {
    fn describe(&self) -> String {
        format!(
            "My name is {}, language is {}.  My grayscale is {}.",
            self.animal.name, self.animal.language, self.grayscale
        )
    }
}

impl Describable for Dog {
    fn describe(&self) -> String {
        format!(
            "My name is {}, language is {}.  My RGB colors are {}, {}, {}.",
            self.animal.name, self.animal.language, self.red, self.green, self.blue
        )
    }
}

struct MyAnimals (
    Cat,
    Dog
);

fn main() {
    let sylvester = Cat {
        animal: Animal {
            name: "Sylvester".to_string(),
            language: "meow".to_string(),
        },
        grayscale: 128,
    };
    let fido = Dog {
        animal: Animal {
            name: "Fido".to_string(),
            language: "bark".to_string(),
        },
        red: 11,
        green: 22,
        blue: 33,
    };
    let animals = MyAnimals(sylvester, fido);
    println!("{}", animals.0.describe());
    println!("{}", animals.1.describe());
}
