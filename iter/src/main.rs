use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NSD: usize = 3; // For example, 3 dimensions (x, y, z)

#[derive(Debug)]
struct Vertex {
    x: f64,
    y: f64,
    z: f64,
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vertex {:.6e} {:.6e} {:.6e}\n", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct OuterLoop {
    vertices: Vec<Vertex>,
}

impl fmt::Display for OuterLoop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vertices_str: Vec<String> = self.vertices.iter().map(|v| v.to_string()).collect();
        write!(f, "\nouter loop\n{}\nendloop", vertices_str.join(""))
    }
}

#[derive(Debug)]
struct Facet {
    normal: Vertex,
    outer_loop: OuterLoop,
}

impl fmt::Display for Facet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "facet normal {}\n    {}", self.normal, self.outer_loop)
    }
}

fn read_stl<P: AsRef<Path>>(path: P) -> io::Result<Vec<Facet>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut facets = Vec::new();
    let mut current_facet: Option<Facet> = None;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.starts_with("facet normal") {
            // Parse the normal vector
            // example:
            // facet normal 0.000000e+00 -1.000000e+00 0.000000e+00
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            let normal = Vertex {
                x: parts[2].parse().unwrap(), // 2 -> x-index
                y: parts[3].parse().unwrap(), // 3 -> y-index
                z: parts[4].parse().unwrap(), // 4 -> z-index
            };
            current_facet = Some(Facet {
                normal,
                outer_loop: OuterLoop {
                    vertices: Vec::new(),
                },
            });
        } else if trimmed.starts_with("vertex") {
            if let Some(ref mut facet) = current_facet {
                // Parse the vertex
                // example
                // vertex 0.000000e+00 0.000000e+00 1.000000e+00
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                let vertex = Vertex {
                    x: parts[1].parse().unwrap(), // 1 -> x-index
                    y: parts[2].parse().unwrap(), // 2 -> y-index
                    z: parts[3].parse().unwrap(), // 3 -> z-index
                };
                facet.outer_loop.vertices.push(vertex);
            }
        } else if trimmed == "endfacet" {
            if let Some(facet) = current_facet.take() {
                facets.push(facet);
            }
        }
    }

    Ok(facets)
}

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let vertex = Vertex {
        x: 1.0,
        y: 0.0,
        z: 1.0,
    };

    println!("{}", vertex);


    let facets = read_stl("/Users/chovey/autotwin/automesh/tests/input/single.stl")?;

    for facet in facets {
        // println!("{:?}", facet);
        println!("{}", facet);
    }

    let aa: Vec<f64> = vec![1.0, 2.0, 3.0];
    println!("aa {:?}", aa);

    let mut aa2 = Vec::new();
    aa2.push(2.0);
    aa2.push(3.0);
    aa2.push(4.0);
    println!("aa2 {:?}", aa2);

    // Create a new vector of length NSD, initialized with 0.0
    let vec_nsd: Vec<f64> = vec![0.0; NSD];

    // Print the vector
    println!("vec_nsd {:?}", vec_nsd);

    // Given scale as usize
    let scale: usize = 2;

    // Convert usize to f64
    let scale_f64: f64 = scale as f64;

    let bb: Vec<f64> = aa.iter().map(|&x| x * scale_f64).collect();
    println!("bb {:?}", bb);

    let cc: Vec<f64> = aa.iter().map(|&x| x * x).collect();
    println!("cc {:?}", cc);

    let dd: Vec<f64> = aa.iter().map(|&x| square(x)).collect();
    println!("dd {:?}", dd);

    let ee: Vec<f64> = dd.iter().map(|&x| square_root(x)).collect();
    println!("ee {:?}", ee);

    Ok(())
}

fn square(x: f64) -> f64 {
    x * x
}

fn square_root(x: f64) -> f64 {
    x.sqrt()
}
