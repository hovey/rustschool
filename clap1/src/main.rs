use clap::{Parser, Subcommand};
use std::time::Instant;

macro_rules! about {
    () => {
        format!(
            "

     @@@@@@@@@@@@@@@@
      @@@@  @@@@@@@@@@
     @@@@  @@@@@@@@@@@
    @@@@  @@@@@@@@@@@@    \x1b[1;4m{}: Automatic mesh generation\x1b[0m
      @@    @@    @@      {}
      @@    @@    @@      {}
    @@@@@@@@@@@@  @@@
    @@@@@@@@@@@  @@@@
    @@@@@@@@@@ @@@@@ @
     @@@@@@@@@@@@@@@@",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_AUTHORS").split(":").collect::<Vec<&str>>()[0],
            env!("CARGO_PKG_AUTHORS").split(":").collect::<Vec<&str>>()[1]
        )
    };
}

#[derive(Parser)]
#[command(about = about!(),  arg_required_else_help = true, version)]
struct Cli {
    #[command(subcommand)]
    // command: Commands,
    command: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    /// Converts between mesh or segmentation file types
    Convert {
        #[command(subcommand)]
        subcommand: ConvertSubcommand,
    },
    /// Prints a greeting message
    Greet {
        /// Name of the person to greet
        name: String,
    },
    /// Add two numbers
    Add {
        /// The first number
        a: f64,
        /// The second number
        b: f64,
    },
    /// Subtract two numbers
    Sub {
        a: f64,
        b: f64,
    },
}

#[derive(Subcommand)]
enum ConvertSubcommand {
    /// Convert mesh file types (inp) -> (exo | mesh | stl | vtk)
    Mesh(ConvertMeshArgs),
    /// Convert segmentation file types (npy | spn) -> (npy | spn)
    Segmentation(ConvertSegmentationArgs),
}


#[derive(clap::Args)]
struct ConvertMeshArgs {
    /// Mesh (inp) input file
    #[arg(long, short, value_name = "FILE")]
    input: String,

    /// Mesh (exo | mesh | stl | vtk) output file
    #[arg(long, short, value_name = "FILE")]
    output: String,

    /// Pass to quiet the terminal output
    #[arg(action, long, short)]
    quiet: bool,
}

#[derive(clap::Args)]
struct ConvertSegmentationArgs {
    /// Segmentation (npy | spn) input file
    #[arg(long, short, value_name = "FILE")]
    input: String,

    /// Segmentation (npy | spn) output file
    #[arg(long, short, value_name = "FILE")]
    output: String,

    /// Number of voxels in the x-direction
    #[arg(
        long,
        short = 'x',
        value_name = "NEL",
        long_help = "Specifies the number of voxels in the x-direction.\n\
                     Required for spn input format conversion.\n\
                     Example: --nelx 100"
    )]
    nelx: Option<usize>,

    /// Number of voxels in the y-direction
    #[arg(
        long,
        short = 'y',
        value_name = "NEL",
        long_help = "Specifies the number of voxels in the y-direction.\n\
                     Required for spn input format conversion.\n\
                     Example: --nely 200"
    )]
    nely: Option<usize>,

    /// Number of voxels in the z-direction
    #[arg(
        long,
        short = 'z',
        value_name = "NEL",
        long_help = "Specifies the number of voxels in the z-direction.\n\
                     Required for spn input format conversion.\n\
                     Example: --nelz 300"
    )]
    nelz: Option<usize>,

    /// Pass to quiet the terminal output
    #[arg(action, long, short)]
    quiet: bool,
}

fn main() {
    let time = Instant::now();
    let cli = Cli::parse();
    let mut is_quiet = false;
    match cli.command {
        Some(Commands::Convert { subcommand }) => match subcommand {
            ConvertSubcommand::Mesh(args) => {
                is_quiet = args.quiet;
                convert_mesh(args.input, args.output, args.quiet);
            }
            ConvertSubcommand::Segmentation(args) => {
                is_quiet = args.quiet;
                convert_segmentation(args.input, args.output, args.nelx, args.nely, args.nelz, args.quiet);
            }
        }
        Some(Commands::Greet { name }) => {
            println!("Hello {}", name);
        }
        Some(Commands::Add { a, b }) => {
            let result = a + b;
            println!("Result: {}", result);
        }
        Some(Commands::Sub { a, b }) => {
            let result = a - b;
            println!("Result: {}", result);
        }
        None => (),
    }
    if !is_quiet {
        println!("       \x1b[1;98mTotal\x1b[0m {:?}", time.elapsed());
    }
}


fn convert_mesh(
    input: String,
    output: String,
    quiet: bool,
) {
    println!("function: convert");
    println!("  input: {input}");
    println!("  output: {output}");
    println!("  quiet: {quiet}")
}

fn convert_segmentation(
    input: String,
    output: String,
    nelx: Option<usize>,
    nely: Option<usize>,
    nelz: Option<usize>,
    quiet: bool,
) {
    println!("function: convert");
    println!("  input: {input}");
    println!("  output: {output}");
    println!("  nelx: {:?}", nelx);
    println!("  nely: {:?}", nely);
    println!("  nelz: {:?}", nelz);
    println!("  quiet: {quiet}")
}
