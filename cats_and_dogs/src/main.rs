// Output:
// My name is Sylvester, language is meow.  My grayscale is 128.
// My name is Fido, language is bark.  My RGB colors are 11, 22, 33.

trait Describable {
    fn speak(&self) -> String {
        // default implementation
        "Uninplemented".to_string()
    }
}

impl Describable for Cat {
    fn speak(&self) -> String {
        format!(
            "{}  My name is {}, My grayscale is {}.",
            greeting(&self.persona.language),
            self.persona.name,
            self.grayscale
        )
    }
}

impl Describable for Dog {
    fn speak(&self) -> String {
        format!(
            "{}  My name is {}.  My RGB colors are {}, {}, {}.",
            greeting(&self.persona.language),
            self.persona.name,
            self.color.red,
            self.color.green,
            self.color.blue
        )
    }
}

fn print_description(item: &impl Describable) {
    println!("{}", item.speak());
}

enum Language {
    Catish,
    Dogish,
    // English,
    // Frenchish,
    // Spanish,
}

fn greeting(language: &Language) -> String {
    match language {
        Language::Catish => "Hello, my language is 'meow'.".to_string(),
        Language::Dogish => "Hello, my language is 'bark'.".to_string(),
        // Language::English => "Hello.".to_string(),
        // Language::Frenchish => "Bonjour.".to_string(),
        // Language::Spanish => "Hola.".to_string(),
    }
}

// Strategy 1
struct Persona {
    name: String,
    language: Language,
}

struct Color {
    red: u32,
    green: u32,
    blue: u32,
}

// Rust does not support struct inheritance.  Rather, Rust supports struct composition.

struct Cat {
    persona: Persona,
    grayscale: u32,
}
struct Dog {
    persona: Persona,
    color: Color,
}

fn cat_factory(name: String, grayscale: u32) -> Cat {
    Cat {
        persona: Persona {
            name,
            language: Language::Catish,
        },
        grayscale,
    }
}

fn dog_factory(name: String, color: Color) -> Dog {
    Dog {
        persona: Persona {
            name,
            language: Language::Dogish,
        },
        color,
    }
}

fn main() {
    // let sylvester = Cat {
    //     persona: Persona {
    //         name: "Sylvester".to_string(),
    //         language: Language::Catish,
    //     },
    //     grayscale: 128,
    // };

    let sylvester = cat_factory("Sylvester".to_string(), 128);

    // let fido = Dog {
    //     persona: Persona {
    //         name: "Fido".to_string(),
    //         language: Language::Dogish,
    //     },
    //     color: Color {
    //         red: 11,
    //         green: 22,
    //         blue: 33,
    //     },
    // };

    let color_defaut = Color {
        red: 1,
        green: 2,
        blue: 3,
    };

    let fido = dog_factory("Fido".to_string(), color_defaut);

    let spot = dog_factory(
        "Spot".to_string(),
        Color {
            red: 11,
            green: 22,
            blue: 33,
        },
    );
    println!("Method 1:");
    print_description(&sylvester);
    print_description(&fido);
    print_description(&spot);

    println!("Method 2:");
    println!("{}", sylvester.speak());
    println!("{}", fido.speak());
    println!("{}", spot.speak());

    // How to do this?
    // let animals = FarmRoster { items: vec![&sylvester, &fido]}

    // let animals = (&sylvester, &fido);
    // for item in animals {
    //     print_description(&item);
    // }
}

// References:
// https://stackoverflow.com/questions/32552593/is-it-possible-for-one-struct-to-extend-an-existing-struct-keeping-all-the-fiel
//
// Using Trait Objects That Allow for Values of Diferent Types
// https://doc.rust-lang.org/book/ch17-02-trait-objects.html
