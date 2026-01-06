use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{Write, Result};
use std::fs::File;
use std::ops::Sub;

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
#[derive(Debug, Clone)]
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

impl Sub for Point3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Applies Laplacian smoothing to a set of vertices in a mesh.
fn apply_laplacian_smoothing(
    mesh: &mut HexMesh,
    smoothable_vertices: &[usize],
    iterations: u32,
) {
    // 1. Build adjacency list for the entire mesh to find neighbors quickly.
    let mut adj = vec![Vec::<usize>::new(); mesh.vertices.len()];
    for cell in &mesh.cells {
        let v = cell.vertices;
        // Simplified: just connect every vertex in a cell to every other vertex in that cell.
        // A more correct approach would be to only connect vertices that share an edge.
        // For grid-aligned hexes, this is good enough.
        for i in 0..8 {
            for j in (i + 1)..8 {
                adj[v[i]].push(v[j]);
                adj[v[j]].push(v[i]);
            }
        }
    }
    // Make neighbors unique
    for neighbors in adj.iter_mut() {
        neighbors.sort_unstable();
        neighbors.dedup();
    }

    // 2. Loop for the specified number of iterations.
    for _ in 0..iterations {
        let mut new_positions = HashMap::new();

        // 3. For each smoothable vertex, calculate its new position (the centroid of its neighbors).
        for &v_idx in smoothable_vertices {
            let neighbors = &adj[v_idx];
            if neighbors.is_empty() {
                continue;
            }

            let mut centroid = Point3D::new(0.0, 0.0, 0.0);
            for &neighbor_idx in neighbors {
                let p = mesh.vertices[neighbor_idx];
                centroid.x += p.x;
                centroid.y += p.y;
                centroid.z += p.z;
            }
            centroid.x /= neighbors.len() as f64;
            centroid.y /= neighbors.len() as f64;
            centroid.z /= neighbors.len() as f64;

            new_positions.insert(v_idx, centroid);
        }

        // 4. After calculating all new positions, update the mesh.
        for (v_idx, new_pos) in new_positions {
            mesh.vertices[v_idx] = new_pos;
        }
    }
}

/// Finds the indices of vertices that are "smoothable".
///
/// Smoothable vertices are those in the padding layers that are not on the outer projected surface.
/// This is done by taking all vertices and removing the "fixed" ones.
/// Fixed vertices are (1) the vertices from the original core mesh and (2) the vertices on the final outer surface.
fn find_smoothable_vertices(
    padded_mesh: &HexMesh,
    original_mesh: &HexMesh,
) -> Vec<usize> {
    // 1. Get the indices of the final outer surface vertices.
    let outer_surface_indices: HashSet<usize> =
        find_outer_surface_vertices(padded_mesh).into_iter().collect();

    // 2. Get the indices of the original core mesh vertices within the padded mesh's vertex list.
    // We do this by creating a set of the 3D coordinates of the original vertices.
    let original_vertex_coords: HashSet<_> = original_mesh.vertices.iter().map(|p| {
        ((p.x * 100.0) as i64, (p.y * 100.0) as i64, (p.z * 100.0) as i64)
    }).collect();

    let mut original_core_indices: HashSet<usize> = HashSet::new();
    for (idx, p) in padded_mesh.vertices.iter().enumerate() {
        let p_coord = ((p.x * 100.0) as i64, (p.y * 100.0) as i64, (p.z * 100.0) as i64);
        if original_vertex_coords.contains(&p_coord) {
            original_core_indices.insert(idx);
        }
    }
    
    // 3. Combine the two sets of fixed vertices.
    let fixed_vertices: HashSet<usize> = outer_surface_indices
        .union(&original_core_indices)
        .cloned()
        .collect();

    // 4. A smoothable vertex is any vertex that is not fixed.
    let mut smoothable_vertices = Vec::new();
    for i in 0..padded_mesh.vertices.len() {
        if !fixed_vertices.contains(&i) {
            smoothable_vertices.push(i);
        }
    }

    smoothable_vertices
}

/// Projects a set of vertices of a mesh onto a sphere.
fn project_vertices_to_sphere(
    mesh: &mut HexMesh,
    vertex_indices: &[usize],
    sphere_radius: f64,
    sphere_center: Point3D,
) {
    for &idx in vertex_indices {
        let p = mesh.vertices[idx];
        let vector_from_center = p - sphere_center;
        let projected_point_vector = vector_from_center.normalize().scale(sphere_radius);
        
        // The result is a vector from origin, so we need to translate it back if center is not origin
        let projected_point = Point3D {
            x: sphere_center.x + projected_point_vector.x,
            y: sphere_center.y + projected_point_vector.y,
            z: sphere_center.z + projected_point_vector.z,
        };

        mesh.vertices[idx] = projected_point;
    }
}

/// Finds the indices of vertices on the exterior surface of the mesh.
fn find_outer_surface_vertices(mesh: &HexMesh) -> Vec<usize> {
    // A map from a canonical face representation (a sorted tuple of 4 vertex indices) to its usage count.
    let mut face_counts: HashMap<[usize; 4], u32> = HashMap::new();

    // For each cell, get its 6 faces, create a canonical representation, and count it.
    for cell in &mesh.cells {
        let v = cell.vertices;
        let faces = [
            [v[0], v[1], v[2], v[3]], // bottom
            [v[4], v[5], v[6], v[7]], // top
            [v[0], v[1], v[5], v[4]], // front
            [v[2], v[3], v[7], v[6]], // back
            [v[1], v[2], v[6], v[5]], // right
            [v[0], v[3], v[7], v[4]], // left
        ];

        for mut face in faces {
            // Sort the indices to create a canonical representation for the face.
            face.sort_unstable();
            *face_counts.entry(face).or_insert(0) += 1;
        }
    }

    // Collect all vertices from faces that only appear once.
    let mut outer_vertices: HashSet<usize> = HashSet::new();
    for (face, count) in face_counts {
        if count == 1 {
            for vertex_index in face {
                outer_vertices.insert(vertex_index);
            }
        }
    }

    outer_vertices.into_iter().collect()
}

/// Finds the integer grid coordinates for the first padding layer.
/// These are the unoccupied grid cells immediately adjacent to the existing mesh.
fn find_padding_layer_locations(existing_mesh: &HexMesh) -> Vec<(i32, i32, i32)> {
    let mut occupied_cells: HashSet<(i32, i32, i32)> = HashSet::new();

    // Populate occupied_cells with the grid coordinates of existing hexes
    for cell in &existing_mesh.cells {
        // Find the min (x,y,z) coordinate among the cell's vertices to represent its grid origin
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;

        for &v_idx in &cell.vertices {
            let p = existing_mesh.vertices[v_idx];
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            min_z = min_z.min(p.z);
        }
        occupied_cells.insert((min_x as i32, min_y as i32, min_z as i32));
    }

    let mut padding_locations: HashSet<(i32, i32, i32)> = HashSet::new();
    let neighbors_offset = [
        (-1, 0, 0), (1, 0, 0), (0, -1, 0),
        (0, 1, 0), (0, 0, -1), (0, 0, 1),
    ];

    // For each occupied cell, check its 6 neighbors
    for &(cx, cy, cz) in &occupied_cells {
        for &(ox, oy, oz) in &neighbors_offset {
            let neighbor_coord = (cx + ox, cy + oy, cz + oz);
            // If the neighbor is not occupied, it's a padding location
            if !occupied_cells.contains(&neighbor_coord) {
                padding_locations.insert(neighbor_coord);
            }
        }
    }

    padding_locations.into_iter().collect()
}

/// Creates a new HexMesh by adding padding cells to an existing base mesh.
///
/// The `base_mesh` provides the initial structure.
/// The `padding_locations` are the grid coordinates (i32, i32, i32) where new 1x1x1 cells should be added.
fn create_padded_mesh(base_mesh: &HexMesh, padding_locations: Vec<(i32, i32, i32)>) -> HexMesh {
    let mut new_vertices: Vec<Point3D> = Vec::new();
    let mut new_cells: Vec<Hexahedron> = Vec::new();
    let mut new_vertex_map: HashMap<(i32, i32, i32), usize> = HashMap::new();

    // Helper to get or create a vertex index, similar to the one in add_block
    let mut get_vertex = |x: i32, y: i32, z: i32| {
        *new_vertex_map.entry((x, y, z)).or_insert_with(|| {
            let new_idx = new_vertices.len();
            new_vertices.push(Point3D::new(x as f64, y as f64, z as f64));
            new_idx
        })
    };

    // First, add all cells and vertices from the base mesh to the new mesh
    for cell in &base_mesh.cells {
        let mut remapped_vertices = [0; 8];
        for (i, &v_idx) in cell.vertices.iter().enumerate() {
            let p = base_mesh.vertices[v_idx];
            // Since our base mesh vertices are integer coordinates, we can cast them directly
            remapped_vertices[i] = get_vertex(p.x as i32, p.y as i32, p.z as i32);
        }
        new_cells.push(Hexahedron { vertices: remapped_vertices });
    }

    // Now, add cells for each padding location
    for &(px, py, pz) in &padding_locations {
        // For each padding location, create a 1x1x1 cell
        let v0 = get_vertex(px, py, pz);
        let v1 = get_vertex(px + 1, py, pz);
        let v2 = get_vertex(px + 1, py + 1, pz);
        let v3 = get_vertex(px, py + 1, pz);
        let v4 = get_vertex(px, py, pz + 1);
        let v5 = get_vertex(px + 1, py, pz + 1);
        let v6 = get_vertex(px + 1, py + 1, pz + 1);
        let v7 = get_vertex(px, py + 1, pz + 1);
        new_cells.push(Hexahedron { vertices: [v0, v1, v2, v3, v4, v5, v6, v7] });
    }

    HexMesh { vertices: new_vertices, cells: new_cells }
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

/// Writes a HexMesh to a file in the Abaqus .inp format.
fn write_hex_mesh_to_inp(mesh: &HexMesh, filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;

    // Write nodes
    writeln!(file, "*NODE, NSET=ALLNODES")?;
    for (i, p) in mesh.vertices.iter().enumerate() {
        // Abaqus uses 1-based indexing
        writeln!(file, "{}, {}, {}, {}", i + 1, p.x, p.y, p.z)?;
    }
    writeln!(file, "**")?;

    // Write elements
    writeln!(file, "*ELEMENT, TYPE=C3D8, ELSET=EB1")?;
    for (i, cell) in mesh.cells.iter().enumerate() {
        // Abaqus uses 1-based indexing
        write!(file, "{}", i + 1)?;
        for &vertex_index in &cell.vertices {
            write!(file, ", {}", vertex_index + 1)?;
        }
        writeln!(file)?;
    }
    writeln!(file, "**")?;
    
    // Properties section
    writeln!(file, "********************************** P R O P E R T I E S ************************")?;
    writeln!(file, "*SOLID SECTION, ELSET=EB1, MATERIAL=Default-Steel")?;
    writeln!(file, "**")?;

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

    // --- First Padding Layer ---
    let padding_locations_1 = find_padding_layer_locations(&hex_mesh);
    println!("Found {} padding locations for the first layer.", padding_locations_1.len());
    let padded_hex_mesh_1_layer = create_padded_mesh(&hex_mesh, padding_locations_1);
    println!(
        "Padded HexMesh (1 layer) created: {} vertices, {} cells",
        padded_hex_mesh_1_layer.vertices.len(),
        padded_hex_mesh_1_layer.cells.len()
    );

    // --- Second Padding Layer ---
    let padding_locations_2 = find_padding_layer_locations(&padded_hex_mesh_1_layer);
    println!("Found {} padding locations for the second layer.", padding_locations_2.len());
    let padded_hex_mesh_2_layers = create_padded_mesh(&padded_hex_mesh_1_layer, padding_locations_2);
    println!(
        "Padded HexMesh (2 layers) created: {} vertices, {} cells",
        padded_hex_mesh_2_layers.vertices.len(),
        padded_hex_mesh_2_layers.cells.len()
    );

    // --- Find Outer Surface for Projection ---
    let outer_vertices_indices = find_outer_surface_vertices(&padded_hex_mesh_2_layers);
    println!("Found {} outer surface vertices for projection.", outer_vertices_indices.len());

    let radius = 5.0;
    let subdivisions = 2;

    // --- Project Outer Vertices to Sphere ---
    let mut projected_mesh = padded_hex_mesh_2_layers.clone();
    let sphere_center = Point3D::new(0.0, 0.0, 0.0);
    project_vertices_to_sphere(&mut projected_mesh, &outer_vertices_indices, radius, sphere_center);
    println!("Projected outer vertices onto sphere.");

    if let Err(e) = write_hex_mesh_to_vtk(&projected_mesh, "projected_mesh.vtk") {
        eprintln!("Error writing projected_mesh.vtk: {}", e);
    } else {
        println!("Successfully wrote projected_mesh.vtk");
    }

    // --- Find Smoothable Vertices ---
    let smoothable_vertices = find_smoothable_vertices(&padded_hex_mesh_2_layers, &hex_mesh);
    println!("Found {} smoothable vertices.", smoothable_vertices.len());

    // --- Apply Laplacian Smoothing ---
    let mut smoothed_mesh = projected_mesh; // Take ownership of the projected mesh
    apply_laplacian_smoothing(&mut smoothed_mesh, &smoothable_vertices, 10); // 10 iterations
    println!("Applied Laplacian smoothing.");

    if let Err(e) = write_hex_mesh_to_vtk(&smoothed_mesh, "smoothed_mesh.vtk") {
        eprintln!("Error writing smoothed_mesh.vtk: {}", e);
    } else {
        println!("Successfully wrote smoothed_mesh.vtk");
    }

    if let Err(e) = write_hex_mesh_to_inp(&smoothed_mesh, "smoothed_mesh.inp") {
        eprintln!("Error writing smoothed_mesh.inp: {}", e);
    } else {
        println!("Successfully wrote smoothed_mesh.inp");
    }

    // --- Icosphere for context ---
    let triangular_mesh = create_icosphere_mesh(radius, subdivisions);
    println!(
        "TriangularMesh (Icosphere) created: {} vertices, {} facets (radius={}, subdivisions={})",
        triangular_mesh.vertices.len(),
        triangular_mesh.facets.len(),
        radius,
        subdivisions
    );
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

    #[test]
    fn test_find_padding_layer_locations() {
        // Manually create a simple mesh with one hexahedron at (0,0,0) to (1,1,1)
        let mut vertices: Vec<Point3D> = Vec::new();
        vertices.push(Point3D::new(0.0, 0.0, 0.0)); // 0
        vertices.push(Point3D::new(1.0, 0.0, 0.0)); // 1
        vertices.push(Point3D::new(1.0, 1.0, 0.0)); // 2
        vertices.push(Point3D::new(0.0, 1.0, 0.0)); // 3
        vertices.push(Point3D::new(0.0, 0.0, 1.0)); // 4
        vertices.push(Point3D::new(1.0, 0.0, 1.0)); // 5
        vertices.push(Point3D::new(1.0, 1.0, 1.0)); // 6
        vertices.push(Point3D::new(0.0, 1.0, 1.0)); // 7

        let cells = vec![
            Hexahedron { vertices: [0, 1, 2, 3, 4, 5, 6, 7] }
        ];

        let single_hex_mesh = HexMesh { vertices, cells };

        let padding_locations = find_padding_layer_locations(&single_hex_mesh);
        // A single hex at (0,0,0) should have 6 padding locations (neighbors)
        assert_eq!(padding_locations.len(), 6, "A single hex should have 6 padding neighbors.");

        // Check for specific locations
        assert!(padding_locations.contains(&(-1, 0, 0)));
        assert!(padding_locations.contains(&(1, 0, 0)));
        assert!(padding_locations.contains(&(0, -1, 0)));
        assert!(padding_locations.contains(&(0, 1, 0)));
        assert!(padding_locations.contains(&(0, 0, -1)));
        assert!(padding_locations.contains(&(0, 0, 1)));
    }

    #[test]
    fn test_create_padded_mesh() {
        // Create a simple mesh with one hexahedron at (0,0,0) to (1,1,1)
        let mut vertices: Vec<Point3D> = Vec::new();
        vertices.push(Point3D::new(0.0, 0.0, 0.0)); // 0
        vertices.push(Point3D::new(1.0, 0.0, 0.0)); // 1
        vertices.push(Point3D::new(1.0, 1.0, 0.0)); // 2
        vertices.push(Point3D::new(0.0, 1.0, 0.0)); // 3
        vertices.push(Point3D::new(0.0, 0.0, 1.0)); // 4
        vertices.push(Point3D::new(1.0, 0.0, 1.0)); // 5
        vertices.push(Point3D::new(1.0, 1.0, 1.0)); // 6
        vertices.push(Point3D::new(0.0, 1.0, 1.0)); // 7

        let cells = vec![
            Hexahedron { vertices: [0, 1, 2, 3, 4, 5, 6, 7] }
        ];

        let single_hex_mesh = HexMesh { vertices, cells };

        let padding_locations = find_padding_layer_locations(&single_hex_mesh);
        let padded_mesh = create_padded_mesh(&single_hex_mesh, padding_locations);

        // Original 1 cell + 6 padding cells = 7 cells
        assert_eq!(padded_mesh.cells.len(), 7, "Padded mesh should have 7 cells.");

        // Original 8 vertices + (6 * 4 new vertices) = 32 unique vertices.
        // Let's re-calculate. 8 original. 6 new cells.
        // Each new cell shares 4 vertices with the original cell.
        // Each new cell also shares vertices with its neighbors.
        // It's a 3x3x1 block on one axis, plus 2 more blocks on other axes.
        // Total is a central cube with 6 cubes attached to its faces.
        // Vertices: 8 for the central cube.
        // 4 new vertices for each of the 6 attached cubes = 24 new vertices. 8 + 24 = 32. This seems right.
        assert_eq!(padded_mesh.vertices.len(), 32, "Padded mesh should have 32 unique vertices.");
    }

    #[test]
    fn test_find_outer_surface_vertices() {
        // Create a simple mesh with one hexahedron
        let mut vertices: Vec<Point3D> = Vec::new();
        vertices.push(Point3D::new(0.0, 0.0, 0.0));
        vertices.push(Point3D::new(1.0, 0.0, 0.0));
        vertices.push(Point3D::new(1.0, 1.0, 0.0));
        vertices.push(Point3D::new(0.0, 1.0, 0.0));
        vertices.push(Point3D::new(0.0, 0.0, 1.0));
        vertices.push(Point3D::new(1.0, 0.0, 1.0));
        vertices.push(Point3D::new(1.0, 1.0, 1.0));
        vertices.push(Point3D::new(0.0, 1.0, 1.0));

        let cells = vec![
            Hexahedron { vertices: [0, 1, 2, 3, 4, 5, 6, 7] }
        ];
        let mesh = HexMesh { vertices, cells };

        let outer_vertices = find_outer_surface_vertices(&mesh);
        // For a single hex, all 8 vertices are on the outer surface.
        assert_eq!(outer_vertices.len(), 8);
    }

    #[test]
    fn test_project_vertices_to_sphere() {
        let mut mesh = HexMesh {
            vertices: vec![Point3D::new(2.0, 0.0, 0.0)],
            cells: vec![],
        };
        let indices_to_project = vec![0];
        let sphere_radius = 5.0;
        let sphere_center = Point3D::new(0.0, 0.0, 0.0);

        project_vertices_to_sphere(&mut mesh, &indices_to_project, sphere_radius, sphere_center);

        let projected_vertex = mesh.vertices[0];
        assert_eq!(projected_vertex.x, 5.0);
        assert_eq!(projected_vertex.y, 0.0);
        assert_eq!(projected_vertex.z, 0.0);

        // Test with another point
        mesh.vertices.push(Point3D::new(3.0, 4.0, 0.0));
        let indices_to_project = vec![1];
        project_vertices_to_sphere(&mut mesh, &indices_to_project, sphere_radius, sphere_center);
        let projected_vertex = mesh.vertices[1];
        // The original vector has length 5, so normalizing and scaling by 5 should result in the same point
        // But due to float precision, we check for closeness
        assert!((projected_vertex.x - 3.0).abs() < 1e-9);
        assert!((projected_vertex.y - 4.0).abs() < 1e-9);
        assert!((projected_vertex.z - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_find_smoothable_vertices() {
        // 1. Create a single-cell core mesh.
        let mut vertices: Vec<Point3D> = Vec::new();
        vertices.push(Point3D::new(0.0, 0.0, 0.0));
        vertices.push(Point3D::new(1.0, 0.0, 0.0));
        vertices.push(Point3D::new(1.0, 1.0, 0.0));
        vertices.push(Point3D::new(0.0, 1.0, 0.0));
        vertices.push(Point3D::new(0.0, 0.0, 1.0));
        vertices.push(Point3D::new(1.0, 0.0, 1.0));
        vertices.push(Point3D::new(1.0, 1.0, 1.0));
        vertices.push(Point3D::new(0.0, 1.0, 1.0));
        let cells = vec![Hexahedron { vertices: [0, 1, 2, 3, 4, 5, 6, 7] }];
        let original_mesh = HexMesh { vertices, cells };

        // 2. Create a 1-layer padded mesh from it.
        let padding_locations = find_padding_layer_locations(&original_mesh);
        let padded_mesh = create_padded_mesh(&original_mesh, padding_locations);

        // 3. Find smoothable vertices.
        let smoothable = find_smoothable_vertices(&padded_mesh, &original_mesh);

        // 4. Assert count. For a 1-layer padding, there are no "internal" padding
        //    vertices to smooth. They are all either original or on the new boundary.
        assert_eq!(smoothable.len(), 0);
    }

    #[test]
    fn test_apply_laplacian_smoothing() {
        // Create a mesh that is a line of 3 vertices and 2 cells (represented as hexes for simplicity)
        let vertices = vec![
            Point3D::new(0.0, 0.0, 0.0), // 0, fixed
            Point3D::new(2.0, 0.0, 0.0), // 1, smoothable
            Point3D::new(4.0, 0.0, 0.0), // 2, fixed
        ];
        // Bogus cells just to create connectivity: 0-1 and 1-2
        let cells = vec![
            Hexahedron { vertices: [0, 1, 0, 0, 0, 0, 0, 0] },
            Hexahedron { vertices: [1, 2, 0, 0, 0, 0, 0, 0] },
        ];
        let mut mesh = HexMesh { vertices, cells };
        let smoothable_vertices = vec![1];

        apply_laplacian_smoothing(&mut mesh, &smoothable_vertices, 1);

        // The middle vertex should move to the average of its neighbors (0 and 2)
        // Centroid = ( (0,0,0) + (4,0,0) ) / 2 = (2,0,0)
        // Wait, the adjacency is cell-based, so vertex 1 is connected to 0 and 2.
        // Let's create a more realistic cell.
        let vertices = vec![
            Point3D::new(0.0, 0.0, 0.0), // 0, neighbor
            Point3D::new(1.0, 0.0, 0.0), // 1, neighbor
            Point3D::new(2.0, 0.0, 0.0), // 2, smoothable
            Point3D::new(3.0, 0.0, 0.0), // 3, neighbor
            Point3D::new(4.0, 0.0, 0.0), // 4, neighbor
        ];
        let cells = vec![
             Hexahedron { vertices: [0,1,2,0,0,0,0,0] },
             Hexahedron { vertices: [2,3,4,0,0,0,0,0] },
        ];
        let mut mesh = HexMesh { vertices, cells };
        let smoothable_vertices = vec![2];
        
        apply_laplacian_smoothing(&mut mesh, &smoothable_vertices, 1);
        
        // Neighbors of vertex 2 are 0,1,3,4
        // Centroid.x = (0+1+3+4)/4 = 8/4 = 2.0
        let smoothed_vertex = mesh.vertices[2];
        assert!((smoothed_vertex.x - 2.0).abs() < 1e-9);
    }
}
