use std::env;
use std::fs;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct Recipe {
    sculpt_binary: String,
    npy_input: String,
    scale_x: f64,
    scale_y: f64,
    scale_z: f64,
    translate_x: f64,
    translate_y: f64,
    translate_z: f64,
    spn_xyz_order: usize,
    yml_schema_version: String,
}


// Invoke this main() function with
// cargo run -- letter_f_autotwin.yml
fn main() {
    // println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let binary_path = &args[0];
    let yml_path = &args[1];

    println!("This Rust application is: {}", binary_path);
    println!("used to read an Autotwin .yml configuration recipe.\n");
    println!("Examples:");
    println!("  {} <some_file>.yml", binary_path);
    println!("  {} letter_f_autotwin.yml\n", binary_path);
    println!("Processing .yml file: {}", yml_path);

    // error messages
    // let err_connot_find = format!("Cannot find file: {}", yml_path);
    let err_cannot_open = format!("Cannot open file: {}", yml_path);
    let err_cannot_read = format!("Cannot read file: {}", yml_path);
    let err_cannot_parse = format!("Cannot parse YAML string to Struct");

    let contents = fs::read_to_string(yml_path)
        .expect(&err_cannot_read);

    println!("Contents of {}:", yml_path);
    println!("{contents}");

    // parse the YAML string to the the Recipe struct
    let recipe: Recipe = serde_yaml::from_str(&contents)
        .expect(&err_cannot_parse);

    // print the parsed contents
    println!("{:?}", recipe);

    // requires that the file extension ends in either .yml or .yaml

    // From https://serde.rs and https://github.com/dtolnay/serde-yaml
    // it appears that serde and serde_yaml crates provide this functionality
    // already.  To the cargo.toml, add these dependencies:
    // [dependencies]
    // serde = { version = "1.0", features = ["derive"] }
    // serde_yaml = "0.9"

    // use the Deserialize trait in serde
    // open the yml file
    // let mut yml_file = fs::File::open(yml_path).expect(&file_io_error);
    // let yml_file = fs::File::open(yml_path);
    // let yml_file = fs::File::open(yml_path).expect(&err_cannot_open);
    // let yml_file = match yml_file {
    //     Ok(yml_file) => yml_file,
    //     Err(error) => {
    //         match error.kind() {
    //             std::io::ErrorKind::NotFound => {
    //                 panic!("{}, {}", &err_cannot_find, error)
    //             }
    //             _ => {
    //                 panic!("{}, {}", &err_cannot_open, error)
    //             }
    //         }
    //     }
    // };

    // // read the file contents into a string
    // let db = fs::read_to_string(yml_file)
    //     .expect(&err_cannot_read);

    // // parse the YAML string into the struct
    // let recipe: Recipe = serde_yaml::from_str(&db);

    // // print the parsed configuration
    // println!("{:?}", recipe);

}
