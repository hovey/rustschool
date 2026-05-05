/// A node in 3D space with x, y, z, coordinats
#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Node {
    /// Creates a new node with the given coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{ x, y, z }
    }
}

/// A linear 3D triangular element defined by three node indices.
#[derive(Debug, Clone)]
pub struct Element {
    pub nodes: [usize; 3],
}

impl Element {
    /// Creates a new triangular element from three node indices.
    pub fn new(nodes: [usize; 3]) -> Self {
        Self { nodes }
    }
}

/// A tesselation consisting of a collection of nodes and triangular elements.
#[derive(Debug, Default)]
pub struct Tesselation {
    pub nodes: Vec<Node>,
    pub elements: Vec<Element>,
}

impl Tesselation {
    /// Creates a new empty tesselation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node to the tesselation and returns its index.
    pub fn add_node(&mut self, node: Node) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    /// Adds a triangular element to the tesselation.
    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tesselation_creation() {
        let mut tess = Tesselation::new();

        // Add 3 nodes to form a single triangle
        let n0 = tess.add_node(Node::new(0.0, 0.0, 0.0));
        let n1 = tess.add_node(Node::new(1.0, 0.0, 0.0));
        let n2 = tess.add_node(Node::new(0.0, 1.0, 0.0));

        assert_eq!(tess.nodes.len(), 3);

        // Add one triangular element
        let element = Element::new([n0, n1, n2]);
        tess.add_element(element);

        assert_eq!(tess.elements.len(), 1);
        assert_eq!(tess.elements[0].nodes.len(), 3);
    }
}