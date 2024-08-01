use ndarray::Array2;
use ndarray_npy::ReadNpyError;
use std::fs::File;
// use std::io;  // unused import

fn read_npy_file(file_path: &str) -> Result<Array2<f64>, ReadNpyError> {
    // open the file
    // let file = File::open(file_path)?;
    let file = File::open(file_path);

    // read the .npy file into an ndarray
    // let array  = ndarray_npy::read_npy(file)?;

    // Ok(array)
}

fn main() {
    match read_npy_file("example.npy") {
        Ok(array) => {
            println!("Successfully read .npy file");
            println!("{:?}", array);
        }
        Err(e) => {
            eprintln!("Failed to read .npy file {:?}", e);
        }
    }
}
