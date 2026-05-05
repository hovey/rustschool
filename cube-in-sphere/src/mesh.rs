/// A node in 3D space with x, y, z coordinates.
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

/// A linear hexahedral finite element defined by eight indices.
/// The indices refer to the position of the node in the Mesh's nodes vector.
#[derive(Debug, Clone)]
pub struct Element {
    pub nodes: [usize; 8],
}

impl Element {
    /// Creates a new hexahedral element from eight node indices.
    pub fn new(nodes: [usize; 8]) -> Self {
        Self { nodes }
    }
}

/// A finite element mesh consisting of a collection of nodes and hexahedral elements.
#[derive(Debug, Default)]
pub struct Mesh {
    pub nodes: Vec<Node>,
    pub elements: Vec<Element>,
}

impl Mesh {
    /// Creates a new empty mesh.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node to the mesh and returns its index.
    pub fn add_node(&mut self, node: Node) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    /// Adds an element to the mesh.
    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_creation() {
        let mut mesh = Mesh::new();

        // Add eight nodes to create a unit cube
        let n0 = mesh.add_node(Node::new(0.0, 0.0, 0.0));
        let n1 = mesh.add_node(Node::new(1.0, 0.0, 0.0));
        let n2 = mesh.add_node(Node::new(1.0, 1.0, 0.0));
        let n3 = mesh.add_node(Node::new(0.0, 1.0, 0.0));
        let n4 = mesh.add_node(Node::new(0.0, 0.0, 1.0));
        let n5 = mesh.add_node(Node::new(1.0, 0.0, 1.0));
        let n6 = mesh.add_node(Node::new(1.0, 1.0, 1.0));
        let n7 = mesh.add_node(Node::new(0.0, 1.0, 1.0));

        assert_eq!(mesh.nodes.len(), 8);
        assert_eq!(n0, 0);
        assert_eq!(n7, 7);

        // Add one hexahedral element
        let element = Element::new([n0, n1, n2, n3, n4, n5, n6, n7]);
        mesh.add_element(element);

        assert_eq!(mesh.elements.len(), 1);
        assert_eq!(mesh.elements[0].nodes[2], n2);

    }


}