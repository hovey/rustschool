// The general point definition in R^2, which will be used for the cell origin
// as well as for points contained in the quadtree.
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

// Boundary of a quadtree cell
struct Rectangle {
    origin: Point,
    width: f32,
    height: f32,
}

// Enum to represent the state of a cell (also known as as a node).
// To implement the recursive ne, nw, sw, se children, a common and idiomatic
// Rust pattern is to use an enum to represent the two states of a quadtree node:
// a leaf with points, or an internal node with children.  To avoid a recursive type
// with infinite size, the children are usually stored in a Box, which allocates them
// on the heap.
enum Cell{
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

// The maximum number of levels that can exist it the quadtree.  The first
// level is level = 0.
const MAX_LEVELS: usize = 2;

// the main Quadtree struct
struct Quadtree {
    boundary: Rectangle,
    level: usize,
    cell: Cell,
}

impl Rectangle {
    // Function to check if a point is within the quadtree's boundary.  If it
    // is, return true, otherwise returen false.
    fn contains(&self, point: &Point) -> bool {
        point.x >= self.origin.x &&
        point.x < self.origin.x + self.width &&
        point.y >= self.origin.y &&
        point.y < self.origin.y + self.height
    }
}


impl Quadtree {
    // Public constructor for the root of the quadtree.
    pub fn new(boundary: Rectangle) -> Self {
        Self::new_with_level(boundary, 0)
    }
    // Internal constructor that includes the level
    fn new_with_level(boundary: Rectangle, level: usize) -> Self {
        Self {
            boundary,
            level,
            cell: Cell::Leaf { points: Vec::new() },
        }
    }
    // Insert a point into the quadtree
    pub fn insert(&mut self, point: Point) -> bool {
        if !self.boundary.contains(&point) {
            return false;
        }

        match &mut self.cell {
            Cell::Leaf { points} => {
                points.push(point);
                if self.level < MAX_LEVELS {
                    self.subdivide();
                }
            }
            Cell::Children { nw, ne, sw, se } => {
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
    // Subdivide a leaf cell into four children cells
    fn subdivide(&mut self) {
        // Take the points from the current leaf, leaving an empty vector in its place.
        let points = if let Cell::Leaf { points } = &mut self.cell {
            std::mem::take(points)
        } else {
            // Should not happen if we only call subdivide on a leaf
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

        let nw = Box::new(Quadtree::new_with_level(nw_boundary, child_level));
        let ne = Box::new(Quadtree::new_with_level(ne_boundary, child_level));
        let sw = Box::new(Quadtree::new_with_level(sw_boundary, child_level));
        let se = Box::new(Quadtree::new_with_level(se_boundary, child_level));
    
        // Replace the leaf with the new children
        self.cell = Cell::Children { nw, ne, sw, se };
    
        // Re-insert all the points that were in the old leaf.
        for p in points { 
            self.insert(p);
        }
    }
}



fn main() {
    println!("Hello, world!");
}
