use std::collections::HashMap;
use std::io::{Write, Result};
use std::fs::File;

/// A 3D point with f64 coordinates.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    /// Creates a new Point3D.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    /// Normalizes the point to a unit vector from the origin.
    pub fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length == 0.0 {
            *self // Avoid division by zero
        } else {
            Point3D::new(self.x / length, self.y / length, self.z / length)
        }
    }

    /// Scales the point by a given factor.
    pub fn scale(&self, factor: f64) -> Self {
        Point3D::new(self.x * factor, self.y * factor, self.z * factor)
    }

    /// Computes the midpoint between two points.
    pub fn midpoint(&self, other: &Point3D) -> Self {
        Point3D::new(
            (self.x + other.x) / 2.0,
            (self.y + other.y) / 2.0,
            (self.z + other.z) / 2.0,
        )
    }
}

/// A hexahedral element defined by 8 vertex indices.
#[derive(Debug, Copy, Clone)]
pub struct Hexahedron {
    pub vertices: [usize; 8],
}

/// A triangular facet defined by 3 vertex indices.
#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub vertices: [usize; 3],
}

/// A mesh composed of hexahedral cells.
#[derive(Debug)]
pub struct HexMesh {
    pub vertices: Vec<Point3D>,
    pub cells: Vec<Hexahedron>,
}

/// A mesh composed of triangular facets (e.g., a surface mesh).
#[derive(Debug)]
pub struct TriangularMesh {
    pub vertices: Vec<Point3D>,
    pub facets: Vec<Triangle>,
}

/// Creates a HexMesh of 56 cells in a 3D cross shape.
/// The mesh is built from 7 blocks of 8 cells each.
fn create_fifty_six_hex_mesh() -> HexMesh {
    let mut vertices: Vec<Point3D> = Vec::new();
    let mut cells: Vec<Hexahedron> = Vec::new();
    let mut vertex_map: HashMap<(i32, i32, i32), usize> = HashMap::new();

    // Define the 7 blocks by the origin of their local 3x3x3 node grid.
    let block_origins = [
        // XY plane cross shape
        (-3, -1, -1), // Left
        (-1, -1, -1), // Center
        (1, -1, -1),  // Right
        (-1, -3, -1), // Bottom
        (-1, 1, -1),  // Top
        // Z-axis additions
        (-1, -1, 1),  // Up (in +z)
        (-1, -1, -3), // Down (in -z)
    ];

    for origin in &block_origins {
        add_block(*origin, &mut vertices, &mut cells, &mut vertex_map);
    }

    HexMesh { vertices, cells }
}

/// Helper to add a single 2x2x2 block of 8 cells to the mesh.
fn add_block(
    origin: (i32, i32, i32),
    vertices: &mut Vec<Point3D>,
    cells: &mut Vec<Hexahedron>,
    vertex_map: &mut HashMap<(i32, i32, i32), usize>,
) {
    let (ox, oy, oz) = origin;

    // Helper to get or create a vertex index for a node in the block's 3x3x3 grid
    let mut get_vertex = |dx, dy, dz| {
        let coord = (ox + dx, oy + dy, oz + dz);
        *vertex_map.entry(coord).or_insert_with(|| {
            let new_idx = vertices.len();
            vertices.push(Point3D::new(coord.0 as f64, coord.1 as f64, coord.2 as f64));
            new_idx
        })
    };

    // Create the 8 cells for this block
    for z in 0..2 {
        for y in 0..2 {
            for x in 0..2 {
                // Get the 8 corners for this specific cell
                let v0 = get_vertex(x, y, z);
                let v1 = get_vertex(x + 1, y, z);
                let v2 = get_vertex(x + 1, y + 1, z);
                let v3 = get_vertex(x, y + 1, z);
                let v4 = get_vertex(x, y, z + 1);
                let v5 = get_vertex(x + 1, y, z + 1);
                let v6 = get_vertex(x + 1, y + 1, z + 1);
                let v7 = get_vertex(x, y + 1, z + 1);
                cells.push(Hexahedron { vertices: [v0, v1, v2, v3, v4, v5, v6, v7] });
            }
        }
    }
}

/// Creates an icosphere mesh.
/// Subdivisions determine the level of detail.
/// Based on http://www.songho.ca/opengl/gl_sphere.html (Icosphere section)
fn create_icosphere_mesh(radius: f64, subdivisions: u32) -> TriangularMesh {
    let mut vertices: Vec<Point3D> = Vec::new();
    let mut facets: Vec<Triangle> = Vec::new();
    let mut middle_point_index_cache: HashMap<(usize, usize), usize> = HashMap::new();

    // Golden ratio constant
    let t = (1.0 + 5.0f64.sqrt()) / 2.0;

    // Create 12 vertices of an icosahedron
    vertices.push(Point3D::new(-1.0,  t, 0.0).normalize().scale(radius));
    vertices.push(Point3D::new( 1.0,  t, 0.0).normalize().scale(radius));
    vertices.push(Point3D::new(-1.0, -t, 0.0).normalize().scale(radius));
    vertices.push(Point3D::new( 1.0, -t, 0.0).normalize().scale(radius));

    vertices.push(Point3D::new(0.0, -1.0,  t).normalize().scale(radius));
    vertices.push(Point3D::new(0.0,  1.0,  t).normalize().scale(radius));
    vertices.push(Point3D::new(0.0, -1.0, -t).normalize().scale(radius));
    vertices.push(Point3D::new(0.0,  1.0, -t).normalize().scale(radius));

    vertices.push(Point3D::new( t, 0.0, -1.0).normalize().scale(radius));
    vertices.push(Point3D::new( t, 0.0,  1.0).normalize().scale(radius));
    vertices.push(Point3D::new(-t, 0.0, -1.0).normalize().scale(radius));
    vertices.push(Point3D::new(-t, 0.0,  1.0).normalize().scale(radius));

    // Create 20 faces (triangles) of icosahedron
    facets.push(Triangle { vertices: [0, 11, 5] });
    facets.push(Triangle { vertices: [0, 5, 1] });
    facets.push(Triangle { vertices: [0, 1, 7] });
    facets.push(Triangle { vertices: [0, 7, 10] });
    facets.push(Triangle { vertices: [0, 10, 11] });
    facets.push(Triangle { vertices: [1, 5, 9] });
    facets.push(Triangle { vertices: [5, 11, 4] });
    facets.push(Triangle { vertices: [11, 10, 2] });
    facets.push(Triangle { vertices: [10, 7, 6] });
    facets.push(Triangle { vertices: [7, 1, 8] });
    facets.push(Triangle { vertices: [3, 9, 4] });
    facets.push(Triangle { vertices: [3, 4, 2] });
    facets.push(Triangle { vertices: [3, 2, 6] });
    facets.push(Triangle { vertices: [3, 6, 8] });
    facets.push(Triangle { vertices: [3, 8, 9] });
    facets.push(Triangle { vertices: [4, 9, 5] });
    facets.push(Triangle { vertices: [2, 4, 11] });
    facets.push(Triangle { vertices: [6, 2, 10] });
    facets.push(Triangle { vertices: [8, 6, 7] });
    facets.push(Triangle { vertices: [9, 8, 1] });

    // Subdivide the faces
    for _ in 0..subdivisions {
        let mut new_facets: Vec<Triangle> = Vec::new();
        for tri in facets.iter() {
            let v = tri.vertices;
            let a = get_middle_point(&mut vertices, &mut middle_point_index_cache, v[0], v[1], radius);
            let b = get_middle_point(&mut vertices, &mut middle_point_index_cache, v[1], v[2], radius);
            let c = get_middle_point(&mut vertices, &mut middle_point_index_cache, v[2], v[0], radius);
            new_facets.push(Triangle { vertices: [v[0], a, c] });
            new_facets.push(Triangle { vertices: [v[1], b, a] });
            new_facets.push(Triangle { vertices: [v[2], c, b] });
            new_facets.push(Triangle { vertices: [a, b, c] });
        }
        facets = new_facets;
    }
    TriangularMesh { vertices, facets }
}

/// Helper function to get or create a midpoint vertex, scaled to the sphere's radius.
fn get_middle_point(
    vertices: &mut Vec<Point3D>,
    cache: &mut HashMap<(usize, usize), usize>,
    p1: usize,
    p2: usize,
    radius: f64,
) -> usize {
    let (small, large) = if p1 < p2 { (p1, p2) } else { (p2, p1) };
    if let Some(&index) = cache.get(&(small, large)) {
        return index;
    }
    let point1 = vertices[p1];
    let point2 = vertices[p2];
    let midpoint = point1.midpoint(&point2).normalize().scale(radius);
    let index = vertices.len();
    vertices.push(midpoint);
    cache.insert((small, large), index);
    index
}

/// Writes a HexMesh to a file in the VTK legacy format.
fn write_hex_mesh_to_vtk(mesh: &HexMesh, filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "# vtk DataFile Version 3.0")?;
    writeln!(file, "Hexahedral Mesh")?;
    writeln!(file, "ASCII")?;
    writeln!(file, "DATASET UNSTRUCTURED_GRID")?;
    writeln!(file, "POINTS {} float", mesh.vertices.len())?;
    for p in &mesh.vertices {
        writeln!(file, "{} {} {}", p.x, p.y, p.z)?;
    }
    let cells_list_size = mesh.cells.len() * (8 + 1);
    writeln!(file, "CELLS {} {}", mesh.cells.len(), cells_list_size)?;
    for cell in &mesh.cells {
        write!(file, "8")?;
        for &vertex_index in &cell.vertices {
            write!(file, " {}", vertex_index)?;
        }
        writeln!(file)?;
    }
    writeln!(file, "CELL_TYPES {}", mesh.cells.len())?;
    for _ in &mesh.cells {
        writeln!(file, "12")?; // VTK_HEXAHEDRON type is 12
    }
    Ok(())
}

/// Writes a TriangularMesh to a file in the VTK legacy format.
fn write_triangular_mesh_to_vtk(mesh: &TriangularMesh, filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "# vtk DataFile Version 3.0")?;
    writeln!(file, "Triangular Mesh")?;
    writeln!(file, "ASCII")?;
    writeln!(file, "DATASET POLYDATA")?;
    writeln!(file, "POINTS {} float", mesh.vertices.len())?;
    for p in &mesh.vertices {
        writeln!(file, "{} {} {}", p.x, p.y, p.z)?;
    }
    let facets_list_size = mesh.facets.len() * (3 + 1);
    writeln!(file, "POLYGONS {} {}", mesh.facets.len(), facets_list_size)?;
    for facet in &mesh.facets {
        write!(file, "3")?;
        for &vertex_index in &facet.vertices {
            write!(file, " {}", vertex_index)?;
        }
        writeln!(file)?;
    }
    Ok(())
}

fn main() {
    let hex_mesh = create_fifty_six_hex_mesh();
    println!(
        "HexMesh created: {} vertices, {} cells",
        hex_mesh.vertices.len(),
        hex_mesh.cells.len()
    );
    let radius = 5.0;
    let subdivisions = 2;
    let triangular_mesh = create_icosphere_mesh(radius, subdivisions);
    println!(
        "TriangularMesh (Icosphere) created: {} vertices, {} facets (radius={}, subdivisions={})",
        triangular_mesh.vertices.len(),
        triangular_mesh.facets.len(),
        radius,
        subdivisions
    );
    if let Err(e) = write_hex_mesh_to_vtk(&hex_mesh, "hex_mesh.vtk") {
        eprintln!("Error writing hex_mesh.vtk: {}", e);
    } else {
        println!("Successfully wrote hex_mesh.vtk");
    }
    if let Err(e) = write_triangular_mesh_to_vtk(&triangular_mesh, "sphere.vtk") {
        eprintln!("Error writing sphere.vtk: {}", e);
    } else {
        println!("Successfully wrote sphere.vtk");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fifty_six_hex_mesh() {
        let hex_mesh = create_fifty_six_hex_mesh();
        assert_eq!(hex_mesh.cells.len(), 56, "Should have 56 hexahedral cells (7 blocks of 8)");
        assert_eq!(hex_mesh.vertices.len(), 135, "Should have 135 unique vertices for the 3D cross shape");
    }

    #[test]
    fn test_create_icosphere_mesh_subdivision_0() {
        let mesh = create_icosphere_mesh(1.0, 0);
        assert_eq!(mesh.vertices.len(), 12);
        assert_eq!(mesh.facets.len(), 20);
    }

    #[test]
    fn test_create_icosphere_mesh_subdivision_1() {
        let mesh = create_icosphere_mesh(1.0, 1);
        assert_eq!(mesh.facets.len(), 80);
        assert_eq!(mesh.vertices.len(), 42);
    }

    #[test]
    fn test_create_icosphere_mesh_subdivision_2() {
        let mesh = create_icosphere_mesh(1.0, 2);
        assert_eq!(mesh.facets.len(), 320);
        assert_eq!(mesh.vertices.len(), 162);
    }
}
