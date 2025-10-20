// use dirs;
use serde::Serialize;
use std::env; // Needed for env::current_dir()
use std::fs::File;
use std::io::Write;
use std::process::Command;

/// Represents a point in 2D space.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Represents an axis-aligned rectangular boundary.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Rectangle {
    pub origin: Point,
    pub width: f64,
    pub height: f64,
}

/// Represents the state of a quadtree node.
// Enum to represent the state of a node.
// To implement the recursive nw, ne, sw, se children, a common and idiomatic
// Rust pattern is to use an enum to represent the two states of a quadtree node:
// a leaf node with points, or an internal node with children.  To avoid a recursive
// type with infinite size, the children are usually stored in a Box, which allocates
// them on the heap.
#[derive(Debug, Serialize)]
pub enum Node {
    /// A leaf node that stores a list of points.
    Leaf { points: Vec<Point> },
    /// An internal node containing four children quadtrees.
    Children {
        nw: Box<Quadtree>,
        ne: Box<Quadtree>,
        sw: Box<Quadtree>,
        se: Box<Quadtree>,
    },
}

/// A Quadtree data structure.
#[derive(Debug, Serialize)]
pub struct Quadtree {
    pub boundary: Rectangle,
    pub level: usize,
    pub level_max: usize,
    pub node: Node,
}

/// A cardinal direction to search for face neighbors, to the north, east, south, west
#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

/// Represents a "hanging edge" in the quadtree.
///
/// A hanging edge is an edge of a larger cell that has one or more vertices
/// of adjacent, smaller cells lying on it.
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct HangingEdge {
    /// The center of the coarse cell that this edge belongs to.
    pub coarse_cell_center: Point,
    /// The first vertex of the coarse edge.
    pub v1: Point,
    /// The second vertex of the coarse edge.
    pub v2: Point,
    /// The list of hanging nodes that lie on this edge.
    /// For a 2:1 balanced tree, this will contain exactly one point.
    pub hanging_nodes: Vec<Point>,
}

impl Rectangle {
    /// Checks if a point is within the rectangle's boundary.
    ///
    /// The check is inclusive of the origin and exclusive of the top and right edges.
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.origin.x
            && point.x < self.origin.x + self.width
            && point.y >= self.origin.y
            && point.y < self.origin.y + self.height
    }
    // // No longer needed
    // // Checks if this rectangle intersects with another rectangle.
    // pub fn intersects(&self, other: &Rectangle) -> bool {
    //     // No intersection if one rectangle is entirely to side of the other
    //     !(self.origin.x > other.origin.x + other.width
    //         || self.origin.x + self.width < other.origin.x
    //         || self.origin.y > other.origin.y + other.height
    //         || self.origin.y + self.height < other.origin.y)
    // }

    /// Gets the vertices of an edge given a direction.
    fn edge_vertices(&self, direction: Direction) -> (Point, Point) {
        let x0 = self.origin.x;
        let y0 = self.origin.y;
        let x1 = x0 + self.width;
        let y1 = y0 + self.height;

        match direction {
            Direction::North => (Point { x: x0, y: y1 }, Point { x: x1, y: y1 }),
            Direction::East => (Point { x: x1, y: y0 }, Point { x: x1, y: y1 }),
            Direction::South => (Point { x: x0, y: y0 }, Point { x: x1, y: y0 }),
            Direction::West => (Point { x: x0, y: y0 }, Point { x: x0, y: y1 }),
        }
    }
}

impl Quadtree {
    /// Creates a new, empty Quadtree with a given boundary and maximum depth.
    ///
    /// # Arguments
    ///
    /// * `boundary` - The axis-aligned boundary of the root node.
    /// * `level_max` - The maximum number of times the tree can be subdivided.
    pub fn new(boundary: Rectangle, level_max: usize) -> Self {
        Self::new_with_level(boundary, 0, level_max)
    }
    // Internal constructor that includes the level
    fn new_with_level(boundary: Rectangle, level: usize, level_max: usize) -> Self {
        Self {
            boundary,
            level,
            level_max,
            node: Node::Leaf { points: Vec::new() },
        }
    }
    /// Inserts a point into the quadtree.
    ///
    /// The point is recursively inserted into the appropriate leaf node.
    /// This function does not trigger subdivision.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to insert.
    ///
    /// # Returns
    ///
    /// `true` if the point is within the quadtree's boundary and was inserted, `false` otherwise.
    pub fn insert(&mut self, point: Point) -> bool {
        if !self.boundary.contains(&point) {
            return false;
        }

        match &mut self.node {
            Node::Leaf { points } => {
                points.push(point);
                // if self.level < self.level_max {
                //     self.subdivide();
                // }
                return true;
            }
            Node::Children { nw, ne, sw, se } => {
                // Create a performant version to avoid redundant boundary checks.
                let center_x = self.boundary.origin.x + self.boundary.width / 2.0;
                let center_y = self.boundary.origin.y + self.boundary.height / 2.0;

                if point.x < center_x {
                    if point.y < center_y {
                        sw.insert(point)
                    } else {
                        nw.insert(point)
                    }
                } else {
                    if point.y < center_y {
                        se.insert(point)
                    } else {
                        ne.insert(point)
                    }
                }
            }
        }
    }
    // Subdivide a leaf node into four children nodes
    pub fn subdivide(&mut self) {
        // Take the points from the current leaf node, leaving an empty vector in its place.
        let points = if let Node::Leaf { points } = &mut self.node {
            std::mem::take(points)
        } else {
            // Should not happen if we only call subdivide on a leaf node
            return;
        };

        let half_width = self.boundary.width / 2.0;
        let half_height = self.boundary.height / 2.0;
        let x = self.boundary.origin.x;
        let y = self.boundary.origin.y;
        let child_level = self.level + 1;

        let nw_boundary = Rectangle {
            origin: Point {
                x,
                y: y + half_height,
            },
            width: half_width,
            height: half_height,
        };
        let ne_boundary = Rectangle {
            origin: Point {
                x: x + half_width,
                y: y + half_height,
            },
            width: half_width,
            height: half_height,
        };
        let sw_boundary = Rectangle {
            origin: Point { x, y },
            width: half_width,
            height: half_height,
        };
        let se_boundary = Rectangle {
            origin: Point {
                x: x + half_width,
                y,
            },
            width: half_width,
            height: half_height,
        };

        let mut nw = Box::new(Quadtree::new_with_level(
            nw_boundary,
            child_level,
            self.level_max,
        ));
        let mut ne = Box::new(Quadtree::new_with_level(
            ne_boundary,
            child_level,
            self.level_max,
        ));
        let mut sw = Box::new(Quadtree::new_with_level(
            sw_boundary,
            child_level,
            self.level_max,
        ));
        let mut se = Box::new(Quadtree::new_with_level(
            se_boundary,
            child_level,
            self.level_max,
        ));
        // Distribute the points of the parent leaf to the new children.
        // Create a performant version to avoid redundant boundary checks.
        let center_x = self.boundary.origin.x + self.boundary.width / 2.0;
        let center_y = self.boundary.origin.y + self.boundary.height / 2.0;

        for p in points {
            if p.x < center_x {
                if p.y < center_y {
                    if let Node::Leaf { points } = &mut sw.node {
                        points.push(p);
                    }
                } else {
                    if let Node::Leaf { points } = &mut nw.node {
                        points.push(p);
                    }
                }
            } else {
                if p.y < center_y {
                    if let Node::Leaf { points } = &mut se.node {
                        points.push(p);
                    }
                } else {
                    if let Node::Leaf { points } = &mut ne.node {
                        points.push(p);
                    }
                }
            }
        }
        // Replace the leaf node with the new children nodes
        self.node = Node::Children { nw, ne, sw, se };
    }

    /// Refines the quadtree by subdividing leaves that contain points.
    ///
    /// This function traverses the tree and subdivides any leaf node that
    /// contains one or more points and has not yet reached `level_max`.
    /// The process is recursive.
    pub fn refine(&mut self) {
        // If the current node is a leaf that contains points and has not reached the
        // level_max, then subdivide it
        if let Node::Leaf { points } = &self.node {
            if !points.is_empty() && self.level < self.level_max {
                self.subdivide();
            }
        }
        // After potential subdivision, the node might now be a `Children` node.
        // If so, recursively refine each child.
        if let Node::Children { nw, ne, sw, se } = &mut self.node {
            nw.refine();
            ne.refine();
            sw.refine();
            se.refine();
        }
    }

    /// Balances the quadtree using the weak balancing condition.
    ///
    /// This is a post-processing step that ensures any two adjacent leaf nodes
    /// (sharing a full edge) differ by at most one level of refinement.
    /// The function iteratively subdivides leaves until the tree is balanced.
    pub fn weak_balance(&mut self) {
        while self.balance_pass_weakly() {
            // The loop continues as long as a pass makes a change.
        }
    }

    /// Recursively collects all immutable references to the leaf nodes in the quadtree.
    fn get_all_leaves(&self) -> Vec<&Quadtree> {
        let mut leaves = Vec::new();
        match &self.node {
            Node::Leaf { .. } => {
                leaves.push(self);
            }
            Node::Children { nw, ne, sw, se } => {
                leaves.append(&mut nw.get_all_leaves());
                leaves.append(&mut ne.get_all_leaves());
                leaves.append(&mut sw.get_all_leaves());
                leaves.append(&mut se.get_all_leaves());
            }
        }
        leaves
    }

    /// Performs a single balancing pass over the quadtree.
    ///
    /// Traverses the tree to find leaves that violate the weak balancing condition
    /// and marks their neighbors for subdivision.
    ///
    /// # Returns
    ///
    /// `true` if any subdivisions were made, `false` otherwise.
    fn balance_pass_weakly(&mut self) -> bool {
        use std::collections::HashSet;

        // We collect immutable leaves first, find neighbors that need subdivision,
        // and then will later re-aquire mutable referneces to subdivide them.
        let leaves = self.get_all_leaves();
        let mut to_subdivide = HashSet::new();

        for leaf in &leaves {
            // `face_neighbors` is not implemented yet, so this part won't trigger.
            let neighbors = self.face_neighbors(&leaf.boundary);
            for neighbor in neighbors {
                if leaf.level > neighbor.level + 1 {
                    // This neighbor needs to be subdivided.
                    // We store a raw pointer to it to avoid borrow checker issues
                    // and to have a unique identifier for the HashSet.
                    to_subdivide.insert(neighbor as *const Quadtree);
                }
            }
        }

        if to_subdivide.is_empty() {
            return false;
        }

        // Now, perform the subdivisions.
        self.subdivide_leaves_by_pointer(&to_subdivide);

        // Return true to indicate that the tree was modified.
        true
    }

    /// Recursively finds and subdivides leaves identified by a set of raw pointers.
    fn subdivide_leaves_by_pointer(
        &mut self,
        to_subdivide: &std::collections::HashSet<*const Quadtree>,
    ) {
        // If the current node is a leaf, check if it needs to be subdivided.
        if let Node::Leaf { .. } = &self.node {
            let self_ptr = self as *const Quadtree;
            if to_subdivide.contains(&self_ptr) && self.level < self.level_max {
                self.subdivide();
            }
        }

        // After potential subdivisions, the node might now be `Children`, so we traverse
        // into them.
        if let Node::Children { nw, ne, sw, se } = &mut self.node {
            nw.subdivide_leaves_by_pointer(to_subdivide);
            ne.subdivide_leaves_by_pointer(to_subdivide);
            sw.subdivide_leaves_by_pointer(to_subdivide);
            se.subdivide_leaves_by_pointer(to_subdivide);
        }
    }

    /// Finds all leaf nodes that share a face (edge) with a given boundary.
    /// This is the new, traversal-based implementation.
    fn face_neighbors<'a>(&'a self, leaf_boundary: &Rectangle) -> Vec<&'a Quadtree> {
        let mut neighbors = Vec::new();
        neighbors.extend(self.find_neighbors_recursive(leaf_boundary, Direction::North));
        neighbors.extend(self.find_neighbors_recursive(leaf_boundary, Direction::East));
        neighbors.extend(self.find_neighbors_recursive(leaf_boundary, Direction::South));
        neighbors.extend(self.find_neighbors_recursive(leaf_boundary, Direction::West));
        neighbors
    }

    /// Recursively finds adjacent leaves in a specific direction using tree traversal.
    fn find_neighbors_recursive<'a>(
        &'a self,
        target_boundary: &Rectangle,
        direction: Direction,
    ) -> Vec<&'a Quadtree> {
        match &self.node {
            Node::Leaf { .. } => vec![], // Base case, cannot descend further
            Node::Children { nw, ne, sw, se } => {
                let center_x = self.boundary.origin.x + self.boundary.width / 2.0;
                let center_y = self.boundary.origin.y + self.boundary.height / 2.0;

                // Determine which child the target boundary is in.
                let target_is_in_north = target_boundary.origin.y >= center_y;
                let target_is_in_west = target_boundary.origin.x < center_x;

                let child_to_search = if target_is_in_north {
                    if target_is_in_west {
                        nw
                    } else {
                        ne
                    }
                } else {
                    if target_is_in_west {
                        sw
                    } else {
                        se
                    }
                };

                match direction {
                    Direction::North => {
                        if !target_is_in_north
                            && target_boundary.origin.y + target_boundary.height == center_y
                        {
                            let northern_child = if target_is_in_west { nw } else { ne };
                            northern_child.get_leaves_on_edge(Direction::South, target_boundary)
                        } else {
                            child_to_search.find_neighbors_recursive(target_boundary, direction)
                        }
                    }
                    Direction::South => {
                        if target_is_in_north && target_boundary.origin.y == center_y {
                            let southern_child = if target_is_in_west { sw } else { se };
                            southern_child.get_leaves_on_edge(Direction::North, target_boundary)
                        } else {
                            child_to_search.find_neighbors_recursive(target_boundary, direction)
                        }
                    }
                    Direction::East => {
                        if target_is_in_west
                            && target_boundary.origin.x + target_boundary.width == center_x
                        {
                            let eastern_child = if target_is_in_north { ne } else { se };
                            eastern_child.get_leaves_on_edge(Direction::West, target_boundary)
                        } else {
                            child_to_search.find_neighbors_recursive(target_boundary, direction)
                        }
                    }
                    Direction::West => {
                        if !target_is_in_west && target_boundary.origin.x == center_x {
                            let western_child = if target_is_in_north { nw } else { sw };
                            western_child.get_leaves_on_edge(Direction::East, target_boundary)
                        } else {
                            child_to_search.find_neighbors_recursive(target_boundary, direction)
                        }
                    }
                }
            }
        }
    }

    /// Helper function to get all leaves on a specific edge of a quadtree node.
    fn get_leaves_on_edge<'a>(
        &'a self,
        edge: Direction,
        target_boundary: &Rectangle,
    ) -> Vec<&'a Quadtree> {
        // First, check for intersection in the transverse driection.
        // For West/East edge, check for y-overlap.
        // For North/South edge, check x-overlap.
        let intersects = match edge {
            Direction::East | Direction::West => {
                // y-overlap check
                self.boundary.origin.y < target_boundary.origin.y + target_boundary.height
                    && self.boundary.origin.y + self.boundary.height > target_boundary.origin.y
            }
            Direction::North | Direction::South => {
                // x-overlap check
                self.boundary.origin.x < target_boundary.origin.x + target_boundary.width
                    && self.boundary.origin.x + self.boundary.width > target_boundary.origin.x
            }
        };

        if !intersects {
            return vec![]; // No overlap, so no neighbors in this branch.
        }

        // If there is overlap, proceed with finding the leaves on the edge
        match &self.node {
            Node::Leaf { .. } => vec![self],
            Node::Children { nw, ne, sw, se } => match edge {
                Direction::North => {
                    // Collect 'north' from nw and ne
                    let mut leaves = nw.get_leaves_on_edge(edge, target_boundary);
                    leaves.extend(ne.get_leaves_on_edge(edge, target_boundary));
                    leaves
                }
                Direction::East => {
                    // Collect 'east' from ne and se
                    let mut leaves = ne.get_leaves_on_edge(edge, target_boundary);
                    leaves.extend(se.get_leaves_on_edge(edge, target_boundary));
                    leaves
                }
                Direction::South => {
                    // Collect 'south' from sw and se
                    let mut leaves = sw.get_leaves_on_edge(edge, target_boundary);
                    leaves.extend(se.get_leaves_on_edge(edge, target_boundary));
                    leaves
                }
                Direction::West => {
                    // Collect 'west' from nw and sw
                    let mut leaves = nw.get_leaves_on_edge(edge, target_boundary);
                    leaves.extend(sw.get_leaves_on_edge(edge, target_boundary));
                    leaves
                }
            },
        }
    }

    /// Finds all hanging edges in the quadtree
    ///
    /// This function traverses the tree and identifies all "hanging edges",
    /// which are edges of a leaf cell that are adjacent to a more refined
    /// (subdivided) cell.
    ///
    /// # Returns
    ///
    /// A `Vec<HangingEdge>` containing all the identified hanging edges.
    pub fn hanging_edges(&self) -> Vec<HangingEdge> {
        let mut hanging_edges = Vec::new();
        self.hanging_edges_recursive(&mut hanging_edges);
        // Note: This simple traversal might find the same edge twice from
        // opposite sides.  For a final implementation, we might consider
        // using a HashSet to store unique edges before returning a Vec
        hanging_edges
    }

    /// Recursive helper function to find hanging edges.
    fn hanging_edges_recursive(&self, hanging_edges: &mut Vec<HangingEdge>) {
        if let Node::Children { nw, ne, sw, se } = &self.node {
            // Check vertical interfaces
            if let Some(edge) = Self::find_interface_hanging_edge(nw, ne, Direction::East) {
                hanging_edges.push(edge);
            }
            if let Some(edge) = Self::find_interface_hanging_edge(sw, se, Direction::East) {
                hanging_edges.push(edge);
            }

            // Check horizontal interfaces
            if let Some(edge) = Self::find_interface_hanging_edge(nw, sw, Direction::South) {
                hanging_edges.push(edge);
            }
            if let Some(edge) = Self::find_interface_hanging_edge(ne, se, Direction::South) {
                hanging_edges.push(edge);
            }

            // Recurse into children
            nw.hanging_edges_recursive(hanging_edges);
            ne.hanging_edges_recursive(hanging_edges);
            sw.hanging_edges_recursive(hanging_edges);
            se.hanging_edges_recursive(hanging_edges);
        }
    }

    /// Checks for a hanging edge between two adjacent nodes and returns it if found.
    fn find_interface_hanging_edge(
        node1: &Quadtree,
        node2: &Quadtree,
        direction_from_node1: Direction,
    ) -> Option<HangingEdge> {
        match (&node1.node, &node2.node) {
            // Case 1: node1 is a leaf, node2 is subdivided.
            (Node::Leaf { .. }, Node::Children { .. }) => {
                let (v1, v2) = node1.boundary.edge_vertices(direction_from_node1);
                let hanging_node = Point {
                    x: (v1.x + v2.x) / 2.0,
                    y: (v1.y + v2.y) / 2.0,
                };
                Some(HangingEdge {
                    coarse_cell_center: node1.center(),
                    v1,
                    v2,
                    hanging_nodes: vec![hanging_node],
                })
            }
            // Case 2: node2 is a leaf, node1 is subdivided.
            (Node::Children { .. }, Node::Leaf { .. }) => {
                let opposite_direction = match direction_from_node1 {
                    Direction::East => Direction::West,
                    Direction::South => Direction::North,
                    _ => unreachable!(),
                };
                let (v1, v2) = node2.boundary.edge_vertices(opposite_direction);
                let hanging_node = Point {
                    x: (v1.x + v2.x) / 2.0,
                    y: (v1.y + v2.y) / 2.0,
                };
                Some(HangingEdge {
                    coarse_cell_center: node2.center(),
                    v1,
                    v2,
                    hanging_nodes: vec![hanging_node],
                })
            }
            // No hanging edge at this interface.
            _ => None,
        }
    }

    /// Computes the dual vertices of the quadtree.
    ///
    /// A dual vertex is located at the center of each leaf cell in the quadtree.
    /// This is the first step in constructing a dual mesh.
    ///
    /// # Returns
    ///
    /// A `Vec<Point>` containing the coordinates of all dual vertices.
    pub fn dual_vertices(&self) -> Vec<Point> {
        let mut vertices = Vec::new();
        self.dual_vertices_recursive(&mut vertices);
        vertices
    }

    /// Recursive helper function to collect the center points fo all leaf nodes.
    fn dual_vertices_recursive(&self, vertices: &mut Vec<Point>) {
        match &self.node {
            Node::Leaf { .. } => {
                // For a leaf node, the dual vertes is at its center.
                let center = Point {
                    x: self.boundary.origin.x + self.boundary.width / 2.0,
                    y: self.boundary.origin.y + self.boundary.height / 2.0,
                };
                vertices.push(center);
            }
            Node::Children { nw, ne, sw, se } => {
                // If it's not a leaf, recurse into the children.
                nw.dual_vertices_recursive(vertices);
                ne.dual_vertices_recursive(vertices);
                sw.dual_vertices_recursive(vertices);
                se.dual_vertices_recursive(vertices);
            }
        }
    }

    /// Returns teh center point of a quadtree's boundary.
    fn center(&self) -> Point {
        Point {
            x: self.boundary.origin.x + self.boundary.width / 2.0,
            y: self.boundary.origin.y + self.boundary.height / 2.0,
        }
    }

    /// Computes the dual edges for the entire quadtree, handing both uniform
    /// and adpative parts of the grid.
    ///
    /// # Returns
    ///
    /// A `Vec<(Point, Point)>` representing the dual edges.
    pub fn dual_edges(&self) -> Vec<(Point, Point)> {
        let mut edges = Vec::new();
        let leaves = self.get_all_leaves();

        // 1. Add edges of same-level neighbors.
        for leaf in &leaves {
            // Find neighbors to the East and South to avoid generating duplicate edges.
            let neighbors_east = self.find_neighbors_recursive(&leaf.boundary, Direction::East);
            let neighbors_south = self.find_neighbors_recursive(&leaf.boundary, Direction::South);

            for neighbor in neighbors_east.iter().chain(neighbors_south.iter()) {
                // Only create an edge if the neighbor is a the same refinement level.
                if leaf.level == neighbor.level {
                    let p1 = leaf.center();
                    let p2 = neighbor.center();
                    edges.push((p1, p2));
                }
            }
        }

        // 2. Add transition edges for adaptive regions using hanging edges.
        let hanging_edges = self.hanging_edges();
        for edge in hanging_edges {
            for hanging_node in edge.hanging_nodes {
                // Add edge from the coase cell's center to the hanging node.
                edges.push((edge.coarse_cell_center.clone(), hanging_node));
            }
        }

        edges
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    /// Given a quadtree (self), writes the structure of the quadtree to a YAML
    /// file to the `scratch_path`, then calls the Python script `visualize_quadtree.py`
    /// to read in the YAML file and visualize the quadtree using MATPLOTLIB, saving
    /// the figure the `scratch_path`.
    pub fn visualize(
        &self,
        scratch_path: &str,
        file_suffix: &str,
        show_dual: bool,
    ) -> Result<(), String> {
        // Helper struct to hold all data for serialization.
        #[derive(Serialize)]
        struct VisualizationData<'a> {
            quadtree: &'a Quadtree,
            dual_vertices: Option<Vec<Point>>,
            dual_edges: Option<Vec<(Point, Point)>>,
        }

        let mut viz_data = VisualizationData {
            quadtree: self,
            dual_vertices: None,
            dual_edges: None,
        };

        if show_dual {
            viz_data.dual_vertices = Some(self.dual_vertices());
            viz_data.dual_edges = Some(self.dual_edges());
        }

        // let yaml_data = self
        //     .to_yaml()
        //     .map_err(|e| format!("Failed to serialize visualization data to YAML: {}", e))?;
        let yaml_data = serde_yaml::to_string(&viz_data)
            .map_err(|e| format!("Failed to serialize visualization data to YAML: {}", e))?;

        // println!("Generated YAML data:\n{}", yaml_data);
        println!("Generated YAML data for '{}'.", file_suffix);

        // Create a temporary YAML output file with the suffix
        let file_name = format!("quadtree_data_{}.yaml", file_suffix);
        let path_file_name = std::path::Path::new(scratch_path).join(&file_name);
        println!("Scratch YAML file path: {:?}", path_file_name);

        let mut file = File::create(&path_file_name)
            .map_err(|e| format!("Failed to create temporary file: {}", e))?;

        file.write_all(yaml_data.as_bytes())
            .map_err(|e| format!("Failed to write YAML to temporary file: {}", e))?;

        // Get the current working directory to find the Python script
        let current_dir =
            env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

        let script_path = current_dir.join("visualize_quadtree.py");

        // Execute the Python script
        let output = Command::new("python") // Use "python" or "python3" depending on your system
            .arg(&script_path)
            .arg(&path_file_name) // Pass the path to the YAML file as an argument
            .output()
            .map_err(|e| format!("Failed to execute Python script: {}", e))?;

        if output.status.success() {
            println!("Python script executed successfully for '{}'.", file_suffix);
            println!(
                "Python stdout:\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
            Ok(())
        } else {
            Err(format!(
                "Python script failed for '{}' with exit code {:?}\nStdout: {}\nStderr: {}",
                file_suffix,
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer scope

    #[test]
    fn test_rectangle_contains() {
        let rect = Rectangle {
            origin: Point { x: 0.0, y: 0.0 },
            width: 10.0,
            height: 10.0,
        };

        assert!(rect.contains(&Point { x: 5.0, y: 5.0 })); // inside
        assert!(rect.contains(&Point { x: 0.0, y: 5.0 })); // on left edge (inclusive)
        assert!(!rect.contains(&Point { x: 10.0, y: 5.0 })); // on right edge (exclusive)
        assert!(rect.contains(&Point { x: 5.0, y: 0.0 })); // on bottom edge (inclusive)
        assert!(!rect.contains(&Point { x: 5.0, y: 10.0 })); // on top edge (exclusive)
        assert!(!rect.contains(&Point { x: -1.0, y: 5.0 })); // outside left
        assert!(!rect.contains(&Point { x: 11.0, y: 5.0 })); // outside left
        assert!(!rect.contains(&Point { x: 5.0, y: -1.0 })); // outside bottom
        assert!(!rect.contains(&Point { x: 5.0, y: 11.0 })); // outside top
    }

    #[test]
    fn test_quadtree_new() {
        let boundary = Rectangle {
            origin: Point { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
        };
        let quadtree = Quadtree::new(boundary.clone(), 2);

        assert_eq!(quadtree.boundary, boundary);
        assert_eq!(quadtree.level, 0);
        assert_eq!(quadtree.level_max, 2);
        assert!(matches!(quadtree.node, Node::Leaf { points } if points.is_empty()));
    }

    #[test]
    fn test_quadtree_insert_single_point_level_max_0() {
        let boundary = Rectangle {
            origin: Point { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
        };
        let mut quadtree = Quadtree::new(boundary, 0);
        let point = Point { x: 50.0, y: 60.0 };

        assert!(quadtree.insert(point.clone()));
        if let Node::Leaf { points } = quadtree.node {
            assert_eq!(points.len(), 1);
            assert_eq!(points[0], point);
        } else {
            panic!("Quadtree should still be a Leaf if level_max is zero.");
        }
    }

    #[test]
    fn test_quadtree_insert_single_point_level_max_1() {
        let boundary = Rectangle {
            origin: Point { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
        };
        let mut quadtree = Quadtree::new(boundary, 1);
        let point = Point { x: 50.0, y: 50.0 }; // Point will be in ne quadrant

        assert!(quadtree.insert(point.clone()));

        // Refine the quadtree to trigger subdivision
        quadtree.refine();

        if let Node::Children { nw, ne, sw, se } = quadtree.node {
            // Assert that all children are Leaf nodes
            assert!(matches!(nw.node, Node::Leaf { .. }));
            assert!(matches!(ne.node, Node::Leaf { .. }));
            assert!(matches!(sw.node, Node::Leaf { .. }));
            assert!(matches!(se.node, Node::Leaf { .. }));

            // Assert that the ne child contains the point
            if let Node::Leaf { points } = &ne.node {
                assert_eq!(points.len(), 1);
                assert_eq!(points[0], point)
            } else {
                panic!("ne child should be a Leaf with the point.")
            }

            // Assert that the other children are empty Lead nodes
            if let Node::Leaf { points } = &nw.node {
                assert!(points.is_empty());
            } else {
                panic!("nw child should be an empty Leaf node")
            }
            if let Node::Leaf { points } = &sw.node {
                assert!(points.is_empty());
            } else {
                panic!("sw child should be an empty Leaf node")
            }
            if let Node::Leaf { points } = &se.node {
                assert!(points.is_empty());
            } else {
                panic!("se child should be an empty Leaf node")
            }
        } else {
            panic!("Quadtree should be a Children node after subdivision with level_max = 1.");
        }
    }

    #[test]
    fn test_weak_balance() {
        // Manually create an unbalanced tree.
        // Setup:
        // L0: root
        // L1: NW, NE, SW, SE (all leaves)
        // L2: NE is subdivided -> NW_NW, NE_NE, NE_SW, NE_SE (leaves)
        // L3: NE_SW is subdivdded -> NE_SW_NW, NE_SW_NE, NE_SW_SW, NE_SW_SE (leaves)
        //
        // Imbalance:
        // The L3 leaf `NE_SW_NW` is face-adjacent to the roof's L1 `NW` leaf.
        // `leaf.level` = 3, `neighbor.level`=1.
        // `3 > 1 + 1` is true, so the L1 `NW` leaf must be subdivided.

        let mut tree = Quadtree::new(
            Rectangle {
                origin: Point { x: 0.0, y: 0.0 },
                width: 4.0,
                height: 4.0,
            },
            4, // level_max
        );

        // 1. Subdivide to create L1 children.
        tree.subdivide();
        let ne_l1 = match &mut tree.node {
            Node::Children { ne, .. } => ne,
            _ => panic!("Tree should have children after subdivide"),
        };

        // 2. Subdivide the L1 NE child to get L2 children.
        ne_l1.subdivide();
        let ne_sw_l2 = match &mut ne_l1.node {
            Node::Children { sw, .. } => sw,
            _ => panic!("NE child should have L2 children"),
        };

        // 3. Subdivide the L2 NE->SW child to get L3 children.
        ne_sw_l2.subdivide();

        // 4. Run the balancing algorithm.
        tree.weak_balance();

        // 5. Verify that the root's NW leaf (originally L1) was subdivided.
        match &tree.node {
            Node::Children { nw, .. } => {
                assert_eq!(nw.level, 1, "The NW child's level should be 1");
                match &nw.node {
                    Node::Children { .. } => {
                        // Success! The NW leaf was correctly subdivided into a Children node.
                    }
                    _ => panic!("NW node should have been subdivided but it is still a Leaf."),
                }
            }
            _ => panic!("Three should be a Children node."),
        }
    }
}
