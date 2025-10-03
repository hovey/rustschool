// use dirs;
use serde::Serialize;
use std::env; // Needed for env::current_dir()
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

/// Represents a point in 2D space.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Represents an axis-aligned rectangular boundary.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Rectangle {
    pub origin: Point,
    pub width: f32,
    pub height: f32,
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

impl Rectangle {
    /// Checks if a point is within the rectangule's boundary.
    ///
    /// The check is inclusive of the origin and exclusive of the top and right edges.
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.origin.x
            && point.x < self.origin.x + self.width
            && point.y >= self.origin.y
            && point.y < self.origin.y + self.height
    }
    // Checks if this rectangle intersects with another rectangle.
    pub fn intersects(&self, other: &Rectangle) -> bool {
        // No intersection if one rectangle is entirely to side of the other
        !(self.origin.x > other.origin.x + other.width
            || self.origin.x + self.width < other.origin.x
            || self.origin.y > other.origin.y + other.height
            || self.origin.y + self.height < other.origin.y)
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
        // and hten will later re-aquire mutable referneces to subdivide them.
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

    /// Recursively finds all leaves that intersect with a given search area.
    fn find_leaves_in_bounds<'a>(
        &'a self,
        search_area: &Rectangle,
        found_leaves: &mut Vec<&'a Quadtree>,
    ) {
        if !self.boundary.intersects(search_area) {
            return;
        }

        match &self.node {
            Node::Leaf { .. } => {
                found_leaves.push(self);
            }
            Node::Children { nw, ne, sw, se } => {
                nw.find_leaves_in_bounds(search_area, found_leaves);
                ne.find_leaves_in_bounds(search_area, found_leaves);
                sw.find_leaves_in_bounds(search_area, found_leaves);
                se.find_leaves_in_bounds(search_area, found_leaves);
            }
        }
    }

    /// Finds all leaf nodes that share a face (edge) with a given boundary.
    ///
    /// This function queries the tree from the root to find neighbors to the
    /// North, South, East, and West of the specified rectangular area.
    ///
    /// # Arguments
    ///
    /// * `leaf_boundary` - The boundary of the leaf for which to find neighbors.
    ///
    /// # Returns
    ///
    /// A vector of references to the neighboring leaf quadtrees.
    fn face_neighbors<'a>(&'a self, leaf_boundary: &Rectangle) -> Vec<&'a Quadtree> {
        let mut neighbors = Vec::new();
        // A small epsilon helps robustly find neighbors by creating a thin search
        // rectangle that slightly overlaps the boundary edge.
        let epsilon = 0.0001 * leaf_boundary.width;

        // North neighbor search area
        let north_search = Rectangle {
            origin: Point {
                x: leaf_boundary.origin.x,
                y: leaf_boundary.origin.y + leaf_boundary.height,
            },
            width: leaf_boundary.width,
            height: epsilon,
        };
        self.find_leaves_in_bounds(&north_search, &mut neighbors);

        // South neighbor search area
        let south_search = Rectangle {
            origin: Point {
                x: leaf_boundary.origin.x,
                y: leaf_boundary.origin.y - epsilon,
            },
            width: leaf_boundary.width,
            height: epsilon,
        };
        self.find_leaves_in_bounds(&south_search, &mut neighbors);

        // East neighbor search area
        let east_search = Rectangle {
            origin: Point {
                x: leaf_boundary.origin.x + leaf_boundary.width,
                y: leaf_boundary.origin.y,
            },
            width: epsilon,
            height: leaf_boundary.height,
        };
        self.find_leaves_in_bounds(&east_search, &mut neighbors);

        // West neighbor search area
        let west_search = Rectangle {
            origin: Point {
                x: leaf_boundary.origin.x - epsilon,
                y: leaf_boundary.origin.y,
            },
            width: epsilon,
            height: leaf_boundary.height,
        };
        self.find_leaves_in_bounds(&west_search, &mut neighbors);

        neighbors
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    pub fn visualize(&self, file_suffix: &str) -> Result<(), String> {
        let yaml_data = self
            .to_yaml()
            .map_err(|e| format!("Failed to serialize quadtree to YAML: {}", e))?;

        // println!("Generated YAML data:\n{}", yaml_data);
        println!("Generated YAML data for '{}'.", file_suffix);

        // let home_dir =
        //     dirs::home_dir().ok_or_else(|| "Could not find the home directory".to_string())?;
        // let custom_temp_dir = home_dir.join("scratch").join("quadtree");
        let temp_dir = env::temp_dir();
        let custom_temp_dir = temp_dir.join("quadtree");

        // Ensure the custom directory exists
        fs::create_dir_all(&custom_temp_dir).map_err(|e| {
            format!(
                "Failed to create custom directory {:?}: {}",
                custom_temp_dir, e
            )
        })?;
        println!("Custom temporary directory: {:?}", custom_temp_dir);

        // Create a temporary YAML output file with the suffix
        let file_name = format!("quadtree_data_{}.yaml", file_suffix);
        let temp_file_path = custom_temp_dir.join(&file_name);
        println!("Temporary YAML file path: {:?}", temp_file_path);

        let mut file = File::create(&temp_file_path)
            .map_err(|e| format!("Failed to create temporary file: {}", e))?;

        file.write_all(yaml_data.as_bytes())
            .map_err(|e| format!("Failed to write YAML to temporary file: {}", e))?;

        // Get the current working directory to find the PYthon script
        let current_dir =
            env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

        let script_path = current_dir.join("visualize_quadtree.py");

        // Execute the Python script
        let output = Command::new("python") // Use "python" or "python3" depending on your system
            .arg(&script_path)
            .arg(&temp_file_path) // Pass the path to the YAML file as an argument
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
        // L3: NE_SW is subdivded -> NE_SW_NW, NE_SW_NE, NE_SW_SW, NE_SW_SE (leaves)
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
