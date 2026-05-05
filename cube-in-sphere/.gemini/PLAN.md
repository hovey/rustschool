# Plan for the project

- [x] Create a mesh.rs for a finite element mesh data structure.  The mesh consists of a collection of nodes in R3.  Each node has a coordinate (x, y, z), which are all float values.  The mesh also consists of elements.  Each element, a linear hexahedral finite element, is a collection of eight non-negative integers, where eaach integer represents a node number. A
- [x] Create a data.rs file that has cube_1 and cube_2, two example mesh files.
- [x] Create a tess.rs for a tesselation.  The tesselation consists of a collection of nodes in R3.  Each node has a coordinate (x, y, z), which are all float values.  The tesselation also consists of elements.  Each element, a linear 3D triangular element, is a collection of three non-negative integers, where each integer represents a node number.
- [ ] Add to data.rs two tesselations, sphere_1 and sphere_2, where sphere_1 is created from the octa_loop00.stl and sphere_2 is created from octa_loop01.stl.  Both of these STL files are in the data folder, which is parallel to the src folder for the project.

## Future Tasks: Octree Implementation

- [ ] **Create `src/octree.rs`**:
    - [ ] Define an `Octree` struct for spatial partitioning.
    - [ ] The octree should cover a 3D bounding box (Min/Max points).
    - [ ] Define a `NodeCargo` enum or trait to allow the octree to hold nodes from either a `Mesh` or a `Tesselation`.
    - [ ] Implement a recursive `insert` method to place nodes into the correct octant.
    - [ ] Implement a `split` mechanism where a leaf node becomes an internal node with 8 children once a capacity threshold is reached.
- [ ] **Spatial Querying**:
    - [ ] Implement a method to find all nodes within a certain radius or bounding box.
- [ ] **Integration**:
    - [ ] Update `main.rs` to initialize an Octree using the nodes from `cube_2` and `sphere_2`.
    - [ ] Verify that all nodes are correctly accounted for within the Octree structure.

Technical Notes for Tomorrow:

* Recursive Structs: In Rust, to have an Octree node that contains other Octree nodes, we'll need to use Box<T> (smart pointers) because the size of a recursive struct must be known at compile time.
* Generics vs Enums: We'll need to decide if the Octree should be generic over the type of node it holds or if we should use an enum to handle both mesh::Node and tess::Node.


### Code Quality Guidelines

- **Avoid Magic Numbers**: Do not use hard-coded literal values in loops or calculations. Instead, define them as local constants, variables, or project-wide constants to improve readability and maintainability.
**Reference Constants**: When a set of values (like coordinates) is used multiple times, define them once (e.g., in an array or constant) and reference that definition throughout the scope.
