use crate::mesh::{Mesh, Node, Element};
use crate::tess::{Tesselation, Node as TessNode, Element as TessElement};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


/// Create a cube centered around the origin.
pub fn cube_1() -> Mesh {
    let mut mesh = Mesh::new();

    // Define eight nodes of the cube
    mesh.add_node(Node::new(-1.0, -1.0, -1.0)); // 0
    mesh.add_node(Node::new( 1.0, -1.0, -1.0)); // 1
    mesh.add_node(Node::new( 1.0,  1.0, -1.0)); // 2
    mesh.add_node(Node::new(-1.0,  1.0, -1.0)); // 3
    mesh.add_node(Node::new(-1.0, -1.0,  1.0)); // 4
    mesh.add_node(Node::new( 1.0, -1.0,  1.0)); // 5
    mesh.add_node(Node::new( 1.0,  1.0,  1.0)); // 6
    mesh.add_node(Node::new(-1.0,  1.0, 1.0)); // 7

    // Create the single hex element using the eight nodes
    mesh.add_element(Element::new([0, 1, 2, 3, 4, 5, 6, 7]));

    mesh
}


/// Create a cube of side length 2, centered at the origin, discretized into 8
/// hexahedral elements (2 elements per size).
pub fn cube_2() -> Mesh {
    let mut mesh = Mesh::new();

    // Define the grid coordinates for a 3x3 mesh
    let coords = [-1.0, 0.0, 1.0];

    // Define 27 nodes in a 3x3x3 grid
    for z in &coords {
        for y in &coords {
            for x in &coords {
                mesh.add_node(Node::new(*x, *y, *z));
            }
        }
    }

    let n_elements = 2;
    // Define eight elements
    for ez in 0..n_elements { // element z-layer
        for ey in 0..n_elements { // element y-row 
            for ex in 0..n_elements { // element x-column

                // Base index of the "bottom-front-left" of this specific element
                let b = ex + (ey * 3) + (ez * 9);

                // Define 8 corners of the hexahedron relative to base 'b'
                // Bottom Layer (+0): 0, 1, 4 (1+3), 3
                // Top Layer    (+9): 9, 10, 13 (10+3), 12
                let nodes = [
                    b + 0, // (0, 0, 0)
                    b + 1, // (1, 0, 0)
                    b + 4, // (1, 1, 0)
                    b + 3, // (0, 1, 0)
                    b + 9, // (0, 0, 1)
                    b + 10, // (1, 0, 1)
                    b + 13, // (1, 1, 1)
                    b + 12, // (0, 1, 1)
                ];

                mesh.add_element(Element::new(nodes));
            }
        }
    }
    mesh
}

fn load_stl(path: &str) -> Tesselation {
    let mut tess = Tesselation::new();
    let file = File::open(path).expect("Unable to open STL file");
    let reader = BufReader::new(file);

    // Maps (x_bits, y_bits, z_bits) to the existing node index
    let mut node_map: HashMap<(u64, u64, u64), usize> = HashMap::new();
    let mut current_triangle_indices = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line").trim().to_lowercase();

        if line.starts_with("vertex") {
            // Split the line and collect coordinates (skip the word "vertex")
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 4 {
                let x: f64 = parts[1].parse().expect("Invalid x coordinate");
                let y: f64 = parts[2].parse().expect("Invalid y coordinate");
                let z: f64 = parts[3].parse().expect("Invalid z coordinate");
            
                // Convert floats to bits to use as a unique key in the HashMap
                let key = (x.to_bits(), y.to_bits(), z.to_bits());
            
                // Check if we have seen this node before
                let idx = *node_map.entry(key).or_insert_with(|| {
                    // If not seen, add it ot the tesselation and the map
                    tess.add_node(TessNode::new(x, y, z))
                });

                current_triangle_indices.push(idx);
            }
        }
        
        // Once we have 3 vertices, create an element
        if current_triangle_indices.len() == 3 {
            tess.add_element(TessElement::new([
                current_triangle_indices[0],
                current_triangle_indices[1],
                current_triangle_indices[2],
            ]));
            current_triangle_indices.clear();
        }
    }

    tess
}

pub fn sphere_1() -> Tesselation {
    load_stl("data/octa_loop00.stl")
}


pub fn sphere_2() -> Tesselation {
    load_stl("data/octa_loop01.stl")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_1_structure() {
        let mesh = cube_1();
        assert_eq!(mesh.nodes.len(), 8);
        assert_eq!(mesh.elements.len(), 1);

        // Check corner node
        assert_eq!(mesh.nodes[0].x, -1.0);
        assert_eq!(mesh.nodes[6].x, 1.0);
        assert_eq!(mesh.nodes[6].z, 1.0);
    }

    #[test]
    fn test_cube_2_structure() {
        let mesh = cube_2();

        // 3x3x3 grid = 27 nodes
        assert_eq!(mesh.nodes.len(), 27);
        // 2x2x2 elements = 8 elements
        assert_eq!(mesh.elements.len(), 8);

        // Check the very first node (should be at -1, -1, -1)
        assert_eq!(mesh.nodes[0].x, -1.0);
        assert_eq!(mesh.nodes[0].y, -1.0);
        assert_eq!(mesh.nodes[0].z, -1.0);

        // Check the center node (index 13: 1 + 1*3 + 1*9)
        assert_eq!(mesh.nodes[13].x, 0.0);
        assert_eq!(mesh.nodes[13].y, 0.0);
        assert_eq!(mesh.nodes[13].z, 0.0);

        // Check the last node (index 26)
        assert_eq!(mesh.nodes[26].x, 1.0);
        assert_eq!(mesh.nodes[26].y, 1.0);
        assert_eq!(mesh.nodes[26].z, 1.0);

        // Verify connectivity of the first element (lower-front-left octant)
        // Should use nodes indices like [0, 1, 4, 3, 9, 10, 13, 12]
        let e0 = &mesh.elements[0];
        assert_eq!(e0.nodes[0], 0);
        assert_eq!(e0.nodes[1], 1);
        assert_eq!(e0.nodes[4], 9);  // Note: Following our corrected offset logic
        assert_eq!(e0.nodes[6], 13); // Center node is corner of first octant
    }
}