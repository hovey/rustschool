/// A cargo program that operates on a .yml file recipe.  From the recipe
/// in the .yml file, creates a Recipe struct.
///
/// Examples:
///  cargo run -- letter_f_autotwin.yml
///  cargo run -- ~/autotwin/mesh/tests/files/letter_f_autotwin.yml


use std::env;
use std::fs;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;
use serde::Deserialize;
use dirs::home_dir;


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

/// Since the tilde ('~') character is not handled by Rust, but it is used
/// in Linux and macOS, this functions implements an interpretation of the
/// tilde though use of the dirs::home_dir function.
fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~") {
        println!("expand_tilde operating on: {}", path);
        println!{"expanding ~ to: {:?}", home_dir()}
        if let Some(home) = home_dir() {
            let expanded =  home.join(&path[1..]);
            println!("expanded: {:?}", expanded);
            // return home.join(&path[1..]);
            return expanded;
        }
    }
    PathBuf::from(path)
}


/// The main entry point of the Rust program.
fn main() {
    // println!("Hello, world!");
    // TODO: Use the clap command line argment framework https://github.com/clap-rs/clap
    // instead of standard library.
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let binary_path = &args[0];
    let yml_path = &args[1];

    println!("This Rust application is: {}", binary_path);
    println!("used to read an Autotwin .yml recipe.\n");
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
    println!("Converting yaml file contents into internal Recipe struct.");
    let recipe: Recipe = serde_yaml::from_str(&contents)
        .expect(&err_cannot_parse);

    // print the parsed contents
    println!("{:?}", recipe);

    let npy_path = recipe.npy_input;
    println!("Processing .npy file: {}", npy_path);
    
    let npy_path_expanded = expand_tilde(&npy_path);
    println!("(re-)Processing .npy file: {:?}", npy_path_expanded);

    let npy_file = fs::File::open(npy_path).expect("Unable to open .npy file.");
    // let reader = BufReader::new(npy_file);

}
