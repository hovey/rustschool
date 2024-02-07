// Output:
// My name is Sylvester, language is meow.  My grayscale is 128.
// My name is Fido, language is bark.  My RGB colors are 11, 22, 33.

struct Dog {
    red: u32,
    green: u32,
    blue: u32
}

struct Cat {
    grayscale: u32,
}

trait Animal {
    fn get_name(&self) -> String;
    fn get_language(&self) -> String;
}

impl Animal for Dog {
    fn get_name(&self) -> String {
        "Fido".to_string()
    }
    fn get_language(&self) -> String {
        "bark".to_string()
    }
}

impl Animal for Cat {
    fn get_name(&self) -> String {
        "Sylvester".to_string()
    }
    fn get_language(&self) -> String {
        "meow".to_string()
    }
}

trait Describable where Self: Animal {
    fn describe(&self) -> String {
        // default implementation
        "Uninplemented".to_string()
    }
}

impl Describable for Dog {
    fn describe(&self) -> String {
        format!(
            "My name is {}, language is {}.  My RGB colors are {}, {}, {}.",
            self.get_name(), self.get_language(), self.red, self.green, self.blue
        )
    }
}

impl Describable for Cat {
    fn describe(&self) -> String {
        format!(
            "My name is {}, language is {}.  My grayscale is {}.",
            self.get_name(), self.get_language(), self.grayscale
        )
    }
}

fn main() {
    let sylvester = Cat {
        grayscale: 128,
    };
    let fido = Dog {
        red: 11,
        green: 22,
        blue: 33,
    };
    println!("{}", sylvester.describe());
    println!("{}", fido.describe());
}
