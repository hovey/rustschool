// Output:
// My name is Sylvester, language is meow.  My grayscale is 128.
// My name is Fido, language is bark.  My RGB colors are 11, 22, 33.

trait Describable {
    fn describe(&self) -> String {
        // default implementation
        "Uninplemented".to_string()
    }
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

struct Animal {
    name: String,
    language: String,
}

struct Dog {
    animal: Animal,
    red: u32,
    green: u32,
    blue: u32,
}

struct Cat {
    animal: Animal,
    grayscale: u32,
}

fn print_description(item: &impl Describable) {
    println!("{}", item.describe());
}

fn main() {
    let a1 = Animal {
        name: "Sylvester".to_string(),
        language: "meow".to_string(),
    };

    let a2 = Animal {
        name: "Fido".to_string(),
        language: "bark".to_string(),
    };

    let sylvester = Cat {
        animal: a1,
        grayscale: 128,
    };

    let fido = Dog {
        animal: a2,
        red: 11,
        green: 22,
        blue: 33,
    };

    print_description(&sylvester);
    print_description(&fido);
}
