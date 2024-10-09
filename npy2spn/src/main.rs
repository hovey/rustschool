//! This module converts a .npy file to a .spn file, and gives feedback to
//! the user as to the dimensions of the souce .npy file in the command line.
//!
//! Example
//! -------
//!   # build either a debug or release version
//!   cargo build
//!   cargo build --release
//!
//!   # get help
//!   cargo run -- --help
//!
//!   # run the program
//!   cargo run -- --input input_file.npy --output output_file.spn
//!

use clap::Parser;
use ndarray::Array3;
use ndarray_npy::{
    ReadNpyError,
    ReadNpyExt,
    WriteNpyError,
};
use std::{
    fs::File,
    io::{
        BufWriter,
        Error,
        Write,
    },
    path::Path,
    time::Instant,
};

type VoxelData = Array3<u8>;


/// The voxels type.
pub struct Voxels {
    data: VoxelData,
}

pub struct VoxelDimensions {
    x: usize,
    y: usize,
    z: usize,
}

/// Inherent implementation of the voxels type.
impl Voxels {
    /// Constructs and returns a new voxels type from an NPY file.
    pub fn from_npy(file_path: &str) -> Result<Self, ReadNpyError> {
        Ok(Self {
            data: voxel_data_from_npy(file_path)?,
        })
    }
    /// Returns a reference to the internal voxels data.
    pub fn get_data(&self) -> &VoxelData {
        &self.data
    }
    /// Returns the (z, y, x) dimensions of the voxels data.
    pub fn dimensions(&self) -> VoxelDimensions {
        let (z, y, x) = self.data.dim();
        VoxelDimensions {
            x,
            y,
            z,
        }
    }
    /// Writes the internal voxels data to a SPN file.
    pub fn write_spn(&self, file_path: &str) -> Result<(), WriteNpyError> {
        write_voxels_to_spn(self.get_data(), file_path)
    }
}


fn voxel_data_from_npy(file_path: &str) -> Result<VoxelData, ReadNpyError> {
    VoxelData::read_npy(File::open(file_path)?)
}


// fn write_voxels_to_spn(data: &VoxelData, file_path: &str) -> Result<(), WriteNpyError> {
//     let mut file = File::create(file_path)?;  // specify the output file name
// 
//     let mut result = write!(file, "");
//     data.iter()  // goes through ever element, regardless of the array dimension, flattened
//         .for_each(|x|
//             result = writeln!(file, "{}", x.to_string())  // write the line to the file
//         );
//         // is it better to cast .as_bytes instead of .to_string here?
// 
//     Ok(result?)
// }

fn write_voxels_to_spn(data: &VoxelData, file_path: &str) -> Result<(), WriteNpyError> {
    let file = File::create(file_path)?;  // specify the output file name
    let mut writer = BufWriter::new(file);  // Use BufWriter for better performance

    for x in data.iter() {
        writeln!(writer, "{}", x)?;  // Use ? to propagate errors
    }

    Ok(())
}

struct ErrorWrapper {
    message: String,
}

impl std::fmt::Debug for ErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[1;91m{}.\x1b[0m", self.message)
    }
}

impl From<Error> for ErrorWrapper {
    fn from(error: Error) -> ErrorWrapper {
        ErrorWrapper {
            message: error.to_string(),
        }
    }
}

impl From<ReadNpyError> for ErrorWrapper {
    fn from(error: ReadNpyError) -> ErrorWrapper {
        ErrorWrapper {
            message: error.to_string(),
        }
    }
}

impl From<WriteNpyError> for ErrorWrapper {
    fn from(error: WriteNpyError) -> ErrorWrapper {
        ErrorWrapper {
            message: error.to_string(),
        }
    }
}

impl From<String> for ErrorWrapper {
    fn from(message: String) -> ErrorWrapper {
        ErrorWrapper { message }
    }
}


#[derive(Parser)]
#[command(name = "npy2spn")]
#[command(version = "1.0")]
#[command(author = "Chad Hovey <chovey@sandia.gov>")]
#[command(about = "A .npy to .spn file converter", long_about = None)]
struct Args {
    /// Name of the NumPy (.npy) input file.
    #[arg(short, long, value_name = "FILE")]
    input: String,

    /// Name of the SPN (.spn) output file.
    #[arg(short, long, value_name = "FILE")]
    output: String,
}

/// Returns a bold, cyan verison of the input string.
fn bold_cyan(input: &str) -> String {
    // ANSI escape code styles
    let style_start = "\x1b[1;96m";  // bold and cyan
    let style_end = "\x1b[0m";  // reset to default
    format!("{}{}{}", style_start, input, style_end)
}


fn main() -> Result<(), ErrorWrapper> {
    let time_0 = Instant::now();
    let args = Args::parse();

    // Print the input and output files
    // println!("Input file: {}", args.input);
    // println!("Output file: {}", args.output);
    // println!("     \x1b[1;96mReading\x1b[0m {}", args.input);
    // println!("     \x1b[1;96mWriting\x1b[0m {}", args.output);
    println!("     {} {}", bold_cyan("Reading"), args.input);
    
    let input = match Path::new(&args.input)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("npy") => {
            println!("     valid file extension: .npy");
            Voxels::from_npy(&args.input)?
        }
        _ => panic!("Error: input file type must be .npy"),
    };

    let dims = input.dimensions();
    let (nz, ny, nx) = (dims.z, dims.y, dims.x);
    println!("     Read in .npy with dimension (nz, ny, nx) = ({}, {}, {})", nz, ny, nx);

    println!("     {} {}", bold_cyan("Writing"), args.output);

    let _output = match Path::new(&args.output)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("spn") => {
            println!("     valid file extension: .spn");
            input.write_spn(&args.output)?
        }
        _ => panic!("Error: input file type must be .npy or .spn"),
    };

    println!("     {} {:?}", bold_cyan("Done"), time_0.elapsed());

    Ok(())
}
