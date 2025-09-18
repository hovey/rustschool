// The general point definition in R^2, which will be used for the node origin
// as well as for points contained in the quadtree.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// Boundary of a quadtree node
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub origin: Point,
    pub width: f32,
    pub height: f32,
}

// Enum to represent the state of a node.
// To implement the recursive nw, ne, sw, se children, a common and idiomatic
// Rust pattern is to use an enum to represent the two states of a quadtree node:
// a leaf node with points, or an internal node with children.  To avoid a recursive
// type with infinite size, the children are usually stored in a Box, which allocates
// them on the heap.
#[derive(Debug)]
pub enum Node{
    // A leaf node that stores a list of points
    Leaf { points: Vec<Point> },
    // An internal node that has four children
    Children {
        nw: Box<Quadtree>,
        ne: Box<Quadtree>,
        sw: Box<Quadtree>,
        se: Box<Quadtree>,
    },
}

// the main Quadtree struct
#[derive(Debug)]
pub struct Quadtree {
    pub boundary: Rectangle,
    pub level: usize,
    pub max_levels: usize,
    pub node: Node,
}

impl Rectangle {
    // Function to check if a point is within the quadtree's boundary.  If it
    // is, return true, otherwise returen false.
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.origin.x &&
        point.x < self.origin.x + self.width &&
        point.y >= self.origin.y &&
        point.y < self.origin.y + self.height
    }
}

impl Quadtree {
    // Public constructor for the root of the quadtree.
    pub fn new(boundary: Rectangle, max_levels: usize) -> Self {
        Self::new_with_level(boundary, 0, max_levels)
    }
    // Internal constructor that includes the level
    fn new_with_level(boundary: Rectangle, level: usize, max_levels: usize) -> Self {
        Self {
            boundary,
            level,
            max_levels,
            node: Node::Leaf { points: Vec::new() },
        }
    }
    // Insert a point into the quadtree
    pub fn insert(&mut self, point: Point) -> bool {
        if !self.boundary.contains(&point) {
            return false;
        }

        match &mut self.node {
            Node::Leaf { points} => {
                points.push(point);
                if self.level < self.max_levels {
                    self.subdivide();
                }
            }
            Node::Children { nw, ne, sw, se } => {
                // Insert into the correct child, trying each one.
                if nw.insert(point.clone()) {
                    // Point was inserted into nw
                }
                else if ne.insert(point.clone()) {
                    // Point was inserted into ne
                 }
                else if sw.insert(point.clone()) {
                    // Point was inserted into sw
                }
                else if se.insert(point) { 
                    // Point was inserted into se
                }
            }
        }
        true
    }
    // Subdivide a leaf node into four children nodes
    fn subdivide(&mut self) {
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
            origin: Point { x, y: y + half_height },
            width: half_width,
            height: half_height,
        };
        let ne_boundary = Rectangle {
            origin: Point { x: x + half_width, y: y + half_height },
            width: half_width,
            height: half_height,
        };
        let sw_boundary = Rectangle {
            origin: Point { x, y },
            width: half_width,
            height: half_height,
        };
        let se_boundary = Rectangle {
            origin: Point { x: x + half_width, y },
            width: half_width,
            height: half_height,
        };

        let nw = Box::new(Quadtree::new_with_level(nw_boundary, child_level, self.max_levels));
        let ne = Box::new(Quadtree::new_with_level(ne_boundary, child_level, self.max_levels));
        let sw = Box::new(Quadtree::new_with_level(sw_boundary, child_level, self.max_levels));
        let se = Box::new(Quadtree::new_with_level(se_boundary, child_level, self.max_levels));
    
        // Replace the leaf node with the new children nodes
        self.node = Node::Children { nw, ne, sw, se };
    
        // Re-insert all the points that were in the old leaf node.
        for p in points { 
            self.insert(p);
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

        assert!(rect.contains(&Point { x: 5.0, y: 5.0 }));  // inside
        assert!(rect.contains(&Point { x: 0.0, y: 5.0 }));  // on left edge (inclusive)
        assert!(!rect.contains(&Point { x: 10.0, y: 5.0 }));  // on right edge (exclusive)
        assert!(rect.contains(&Point { x: 5.0, y: 0.0 }));  // on bottom edge (inclusive)
        assert!(!rect.contains(&Point { x: 5.0, y: 10.0 }));  // on top edge (exclusive)
        assert!(!rect.contains(&Point { x: -1.0, y: 5.0}));  // outside left
        assert!(!rect.contains(&Point { x: 11.0, y: 5.0}));  // outside left
        assert!(!rect.contains(&Point { x: 5.0, y: -1.0}));  // outside bottom
        assert!(!rect.contains(&Point { x: 5.0, y: 11.0}));  // outside top
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
        assert_eq!(quadtree.max_levels, 2);
        assert!(matches!(quadtree.node, Node::Leaf { points } if points.is_empty()));
    }

    #[test]
    fn test_quadtree_insert_single_point_max_level_0() {
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
            panic!("Quadtree should still be a Leaf if max_levels is zero.");
        }
    }

    #[test]
    fn test_quadtree_insert_single_point_max_level_1() {
        let boundary = Rectangle {
            origin: Point { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
        };
        let mut quadtree = Quadtree::new(boundary, 1);
        let point = Point { x: 50.0, y: 50.0 };  // Point will be in ne quadrant

        assert!(quadtree.insert(point.clone()));
        if let Node::Children { nw, ne, sw, se} = quadtree.node {
            // Assert that all children are Leaf nodes
            assert!(matches!(nw.node, Node::Leaf { .. }));
            assert!(matches!(ne.node, Node::Leaf { .. }));
            assert!(matches!(sw.node, Node::Leaf { .. }));
            assert!(matches!(se.node, Node::Leaf { .. }));

            // Assert that the ne child contains the point
            if let Node::Leaf { points } = ne.node {
                assert_eq!(points.len(), 1);
                assert_eq!(points[0], point)
            } else {
                panic!("ne child should be a Leaf with the point.")
            }

            // Assert that the other children are empty Lead nodes
            if let Node::Leaf { points } = nw.node {
                assert!(points.is_empty());
            } else {
                panic!("nw child should be an empty Leaf node")
            }
            if let Node::Leaf { points } = sw.node {
                assert!(points.is_empty());
            } else {
                panic!("sw child should be an empty Leaf node")
            }
            if let Node::Leaf { points } = se.node {
                assert!(points.is_empty());
            } else {
                panic!("se child should be an empty Leaf node")
            }

        } else {
            panic!("Quadtree should be a Children node after subdivision with max_levels = 1.");
        }
    }
}
