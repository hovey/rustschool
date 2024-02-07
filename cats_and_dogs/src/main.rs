// Output:
// My name is Sylvester, language is meow.  My grayscale is 128.
// My name is Fido, language is bark.  My RGB colors are 11, 22, 33.

struct Dog {
    red: u32,
    green: u32,
    blue: u32,
    name: String,
    language: String
}

struct Cat {
    grayscale: u32,
    name: String,
    language: String
}

trait Animal {
    fn get_name(&self) -> &String;
    fn get_language(&self) -> &String;
    fn get_info(&self) -> String;
}

impl Animal for Dog {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_language(&self) -> &String {
        &self.language
    }
    fn get_info(&self) -> String
    {
        format!(
            "My RGB colors are {}, {}, {}.",
            self.red, self.green, self.blue
        )
    }
}

impl Animal for Cat {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_language(&self) -> &String {
        &self.language
    }
    fn get_info(&self) -> String {
        format!(
            "My grayscale is {}.",
            self.grayscale
        )
    }
}

trait Describable: Animal {
    fn describe(&self) -> String {
        format!(
            "My name is {}, language is {}. {}.",
            self.get_name(), self.get_language(), self.get_info()
        )
    }
}

impl Describable for Cat {}
impl Describable for Dog {}

fn main() {
    let sylvester = Cat {
        grayscale: 128,
        name: "Sylvester".to_string(),
        language: "meow".to_string()
    };
    let fido = Dog {
        red: 11,
        green: 22,
        blue: 33,
        name: "Fido".to_string(),
        language: "bark".to_string()
    };
    println!("{}", sylvester.describe());
    println!("{}", fido.describe());
}
