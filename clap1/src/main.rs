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
    /// Defeatures and creates a new segmentation
    Defeature {
        /// Segmentation input file (npy | spn)
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Defeatured segmentation output file (npy | spn)
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Defeature clusters with less than MIN voxels
        #[arg(long, short, value_name = "MIN")]
        min: usize,

        /// Number of voxels in the x-direction
        #[arg(long, short = 'x', value_name = "NEL")]
        nelx: Option<usize>,

        /// Number of voxels in the y-direction
        #[arg(long, short = 'y', value_name = "NEL")]
        nely: Option<usize>,

        /// Number of voxels in the z-direction
        #[arg(long, short = 'z', value_name = "NEL")]
        nelz: Option<usize>,

        /// Pass to quiet the terminal output
        #[arg(action, long, short)]
        quiet: bool,
    },
    /// Creates a finite element mesh from a segmentation
    Mesh {
        #[command(subcommand)]
        subcommand: MeshSubcommand,
    },
    /// Quality metrics for an existing finite element mesh
    Metrics {
        /// Mesh input file (inp | stl)
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Quality metrics output file (csv | npy)
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Pass to quiet the terminal output
        #[arg(action, long, short)]
        quiet: bool,
    },
    /// Applies smoothing to an existing mesh
    Smooth {
        /// Pass to enable hierarchical control
        #[arg(action, long, short = 'c')]
        hierarchical: bool,

        /// Mesh input file (inp | stl)
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Smoothed mesh output file (exo | inp | mesh | stl | vtk)
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Number of smoothing iterations
        #[arg(default_value_t = 20, long, short = 'n', value_name = "NUM")]
        iterations: usize,

        /// Smoothing method (Laplace | Taubin) [default: Taubin]
        #[arg(long, short, value_name = "NAME")]
        method: Option<String>,

        /// Pass-band frequency (for Taubin only)
        #[arg(default_value_t = 0.1, long, short = 'k', value_name = "FREQ")]
        pass_band: f64,

        /// Scaling parameter for all smoothing methods
        #[arg(default_value_t = 0.6307, long, short, value_name = "SCALE")]
        scale: f64,

        /// Quality metrics output file (csv | npy)
        #[arg(long, value_name = "FILE")]
        metrics: Option<String>,

        /// Pass to quiet the terminal output
        #[arg(action, long, short)]
        quiet: bool,
    }
}

#[derive(Subcommand)]
enum ConvertSubcommand {
    /// Converts mesh file types (inp) -> (exo | mesh | vtk)
    Mesh(ConvertMeshArgs),
    /// Converts segmentation file types (npy | spn) -> (npy | spn)
    Segmentation(ConvertSegmentationArgs),
}

#[derive(Subcommand)]
enum MeshSubcommand {
    /// Creates an all-hexahedral mesh from a segmentation
    Hex(MeshHexArgs),
    /// Creates an all-triangular isosurface from a segmentation
    Tri(MeshTriArgs),
}

#[derive(clap::Args)]
struct ConvertMeshArgs {
    /// Mesh input file (inp)
    #[arg(long, short, value_name = "FILE")]
    input: String,

    /// Mesh output file (exo | mesh | vtk)
    #[arg(long, short, value_name = "FILE")]
    output: String,

    /// Pass to quiet the terminal output
    #[arg(action, long, short)]
    quiet: bool,
}

#[derive(clap::Args)]
struct ConvertSegmentationArgs {
    /// Segmentation input file (npy | spn)
    #[arg(long, short, value_name = "FILE")]
    input: String,

    /// Segmentation output file (npy | spn)
    #[arg(long, short, value_name = "FILE")]
    output: String,

    /// Number of voxels in the x-direction
    #[arg(long, short = 'x', value_name = "NEL")]
    nelx: Option<usize>,

    /// Number of voxels in the y-direction
    #[arg(long, short = 'y', value_name = "NEL")]
    nely: Option<usize>,

    /// Number of voxels in the z-direction
    #[arg(long, short = 'z', value_name = "NEL")]
    nelz: Option<usize>,

    /// Pass to quiet the terminal output
    #[arg(action, long, short)]
    quiet: bool,
}

#[derive(clap::Args)]
struct MeshHexArgs {
    #[command(subcommand)]
    smoothing: Option<MeshSmoothCommands>,

    /// Segmentation input file (npy | spn)
    #[arg(long, short, value_name = "FILE")]
    input: String,

    /// Mesh output file (exo | inp | mesh | vtk)
    #[arg(long, short, value_name = "FILE")]
    output: String,

    /// Defeature clusters with less than NUM voxels
    #[arg(long, short, value_name = "NUM")]
    defeature: Option<usize>,

    /// Number of voxels in the x-direction
    #[arg(long, short = 'x', value_name = "NEL")]
    nelx: Option<usize>,

    /// Number of voxels in the y-direction
    #[arg(long, short = 'y', value_name = "NEL")]
    nely: Option<usize>,

    /// Number of voxels in the z-direction
    #[arg(long, short = 'z', value_name = "NEL")]
    nelz: Option<usize>,

    /// Voxel IDs to remove from the mesh
    #[arg(long, num_args = 1.., short, value_delimiter = ' ', value_name = "ID")]
    remove: Option<Vec<usize>>,

    /// Scaling (> 0.0) in the x-direction, applied before translation
    #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
    xscale: f64,

    /// Scaling (> 0.0) in the y-direction, applied before translation
    #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
    yscale: f64,

    /// Scaling (> 0.0) in the z-direction, applied before translation
    #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
    zscale: f64,

    /// Translation in the x-direction
    #[arg(
        long,
        default_value_t = 0.0,
        allow_negative_numbers = true,
        value_name = "VAL"
    )]
    xtranslate: f64,

    /// Translation in the y-direction
    #[arg(
        long,
        default_value_t = 0.0,
        allow_negative_numbers = true,
        value_name = "VAL"
    )]
    ytranslate: f64,

    /// Translation in the z-direction
    #[arg(
        long,
        default_value_t = 0.0,
        allow_negative_numbers = true,
        value_name = "VAL"
    )]
    ztranslate: f64,

    /// Quality metrics output file (csv | npy)
    #[arg(long, value_name = "FILE")]
    metrics: Option<String>,

    /// Pass to quiet the terminal output
    #[arg(action, long, short)]
    quiet: bool,

    /// Pass to mesh using dualization
    #[arg(action, hide = true, long)]
    dual: bool,
}

#[derive(clap::Args)]
struct MeshTriArgs {
    #[command(subcommand)]
    smoothing: Option<MeshSmoothCommands>,

    /// Segmentation input file (npy | spn)
    #[arg(long, short, value_name = "FILE")]
    input: String,

    /// Mesh output file (exo | inp | mesh | stl | vtk)
    #[arg(long, short, value_name = "FILE")]
    output: String,

    /// Defeature clusters with less than NUM voxels
    #[arg(long, short, value_name = "NUM")]
    defeature: Option<usize>,

    /// Number of voxels in the x-direction
    #[arg(long, short = 'x', value_name = "NEL")]
    nelx: Option<usize>,

    /// Number of voxels in the y-direction
    #[arg(long, short = 'y', value_name = "NEL")]
    nely: Option<usize>,

    /// Number of voxels in the z-direction
    #[arg(long, short = 'z', value_name = "NEL")]
    nelz: Option<usize>,

    /// Voxel IDs to remove from the mesh
    #[arg(long, num_args = 1.., short, value_delimiter = ' ', value_name = "ID")]
    remove: Option<Vec<usize>>,

    /// Scaling (> 0.0) in the x-direction, applied before translation
    #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
    xscale: f64,

    /// Scaling (> 0.0) in the y-direction, applied before translation
    #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
    yscale: f64,

    /// Scaling (> 0.0) in the z-direction, applied before translation
    #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
    zscale: f64,

    /// Translation in the x-direction
    #[arg(
        long,
        default_value_t = 0.0,
        allow_negative_numbers = true,
        value_name = "VAL"
    )]
    xtranslate: f64,

    /// Translation in the y-direction
    #[arg(
        long,
        default_value_t = 0.0,
        allow_negative_numbers = true,
        value_name = "VAL"
    )]
    ytranslate: f64,

    /// Translation in the z-direction
    #[arg(
        long,
        default_value_t = 0.0,
        allow_negative_numbers = true,
        value_name = "VAL"
    )]
    ztranslate: f64,

    /// Quality metrics output file (csv | npy)
    #[arg(long, value_name = "FILE")]
    metrics: Option<String>,

    /// Pass to quiet the terminal output
    #[arg(action, long, short)]
    quiet: bool,

    // There is no dualization for triangles, only hexahedra.
}

#[derive(Subcommand, Debug)]
enum MeshSmoothCommands {
    /// Applies smoothing to the mesh before output
    Smooth {
        /// Pass to enable hierarchical control
        #[arg(action, long, short = 'c')]
        hierarchical: bool,

        /// Number of smoothing iterations
        #[arg(default_value_t = 20, long, short = 'n', value_name = "NUM")]
        iterations: usize,

        /// Smoothing method (Laplace | Taubin) [default: Taubin]
        #[arg(long, short, value_name = "NAME")]
        method: Option<String>,

        /// Pass-band frequency (for Taubin only)
        #[arg(default_value_t = 0.1, long, short = 'k', value_name = "FREQ")]
        pass_band: f64,

        /// Scaling parameter for all smoothing methods
        #[arg(default_value_t = 0.6307, long, short, value_name = "SCALE")]
        scale: f64,
    },
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
                convert_segmentation(
                    args.input,
                    args.output,
                    args.nelx,
                    args.nely,
                    args.nelz,
                    args.quiet,
                );
            }
        },
        Some(Commands::Defeature {
            input,
            output,
            min,
            nelx,
            nely,
            nelz,
            quiet,
        }) => {
            is_quiet = quiet;
            defeature(input, output, min, nelx, nely, nelz, quiet)
        }
        Some(Commands::Mesh { subcommand }) => match subcommand {
            MeshSubcommand::Hex(args) => {
                is_quiet = args.quiet;
                mesh_hex(
                    args.smoothing,
                    args.input,
                    args.output,
                    args.defeature,
                    args.nelx,
                    args.nely,
                    args.nelz,
                    args.remove,
                    args.xscale,
                    args.yscale,
                    args.zscale,
                    args.xtranslate,
                    args.ytranslate,
                    args.ztranslate,
                    args.metrics,
                    args.quiet,
                    args.dual,
                );
            }
            MeshSubcommand::Tri(args) => {
                is_quiet = args.quiet;
                mesh_tri(
                    args.smoothing,
                    args.input,
                    args.output,
                    args.defeature,
                    args.nelx,
                    args.nely,
                    args.nelz,
                    args.remove,
                    args.xscale,
                    args.yscale,
                    args.zscale,
                    args.xtranslate,
                    args.ytranslate,
                    args.ztranslate,
                    args.metrics,
                    args.quiet,
                );
            }
        },
        Some(Commands::Metrics {
            input,
            output,
            quiet,
        }) => {
            is_quiet = quiet;
            metrics(input, output, quiet)
        },
        Some(Commands::Smooth { hierarchical, input, output, iterations, method, pass_band, scale, metrics, quiet }) => {
            is_quiet = quiet;
            smooth(
                hierarchical,
                input,
                output,
                iterations,
                method,
                pass_band,
                scale,
                metrics,
                quiet,
            )
        }
        None => (),
    }
    if !is_quiet {
        println!("       \x1b[1;98mTotal\x1b[0m {:?}", time.elapsed());
    }
}

fn convert_mesh(input: String, output: String, quiet: bool) {
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

fn defeature(
    input: String,
    output: String,
    min: usize,
    nelx: Option<usize>,
    nely: Option<usize>,
    nelz: Option<usize>,
    quiet: bool,
) {
    println!("function: defeature");
    println!("  input: {input}");
    println!("  output: {output}");
    println!("  min: {min}");
    println!("  nelx: {:?}", nelx);
    println!("  nely: {:?}", nely);
    println!("  nelz: {:?}", nelz);
    println!("  quiet: {quiet}")
}

#[allow(clippy::too_many_arguments)]
fn mesh_hex(
    smoothing: Option<MeshSmoothCommands>,
    input: String,
    output: String,
    defeature: Option<usize>,
    nelx: Option<usize>,
    nely: Option<usize>,
    nelz: Option<usize>,
    remove: Option<Vec<usize>>,
    xscale: f64,
    yscale: f64,
    zscale: f64,
    xtranslate: f64,
    ytranslate: f64,
    ztranslate: f64,
    metrics: Option<String>,
    quiet: bool,
    dual: bool,
    // surface: bool,
) {
    println!("function: mesh_hex");
    println!("  smoothing: {:?}", smoothing);
    println!("  input: {input}");
    println!("  output: {output}");
    // println!("  min: {min}");
    println!("  defeature: {:?}", defeature);
    println!("  nelx: {:?}", nelx);
    println!("  nely: {:?}", nely);
    println!("  nelz: {:?}", nelz);
    println!("  remove: {:?}", remove);
    println!("  xscale: {xscale}");
    println!("  yscale: {yscale}");
    println!("  zscale: {zscale}");
    println!("  xtranslate: {xtranslate}");
    println!("  ytranslate: {ytranslate}");
    println!("  ztranslate: {ztranslate}");
    println!("  metrics: {:?}", metrics);
    println!("  quiet: {quiet}");
    println!("  dual: {dual}");
    println!("  quiet: {quiet}");
    // println!("  surface: {surface}");
}

#[allow(clippy::too_many_arguments)]
fn mesh_tri(
    smoothing: Option<MeshSmoothCommands>,
    input: String,
    output: String,
    defeature: Option<usize>,
    nelx: Option<usize>,
    nely: Option<usize>,
    nelz: Option<usize>,
    remove: Option<Vec<usize>>,
    xscale: f64,
    yscale: f64,
    zscale: f64,
    xtranslate: f64,
    ytranslate: f64,
    ztranslate: f64,
    metrics: Option<String>,
    quiet: bool,
    // dual: bool,
    // surface: bool,
) {
    println!("function: mesh_tri");
    println!("  smoothing: {:?}", smoothing);
    println!("  input: {input}");
    println!("  output: {output}");
    // println!("  min: {min}");
    println!("  defeature: {:?}", defeature);
    println!("  nelx: {:?}", nelx);
    println!("  nely: {:?}", nely);
    println!("  nelz: {:?}", nelz);
    println!("  remove: {:?}", remove);
    println!("  xscale: {xscale}");
    println!("  yscale: {yscale}");
    println!("  zscale: {zscale}");
    println!("  xtranslate: {xtranslate}");
    println!("  ytranslate: {ytranslate}");
    println!("  ztranslate: {ztranslate}");
    println!("  metrics: {:?}", metrics);
    println!("  quiet: {quiet}");
    // println!("  dual: {dual}");
    println!("  quiet: {quiet}");
    // println!("  surface: {surface}");
}

fn metrics(input: String, output: String, quiet: bool) {
    println!("function: metrics");
    println!("  input: {input}");
    println!("  output: {output}");
    println!("  quiet: {quiet}");
}

#[allow(clippy::too_many_arguments)]
fn smooth(
    hierarchical: bool,
    input: String,
    output: String,
    iterations: usize,
    method: Option<String>,
    pass_band: f64,
    scale: f64,
    metrics: Option<String>,
    quiet: bool,
) {
    println!("function: smooth");
    println!("  hierarchical: {hierarchical}");
    println!("  input: {input}");
    println!("  output: {output}");
    println!("  iterations: {iterations}");
    println!("  method: {:?}", method);
    println!("  pass_band: {pass_band}");
    println!("  scale: {scale}");
    println!("  metrics: {:?}", metrics);
    println!("  quiet: {quiet}");
}