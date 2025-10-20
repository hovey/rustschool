# Plan for the Quadtree Project

This plan tracks the development of the Rust Quadtree library.

## Note on Collaboration

I will make the code updates manually.  Please provide your sugestions, and I will apply them.

## Context

Use any useful literature and scholarly documents on quadtree definitions, formulation, and implementation, including the following:

@article{livesu2019cinolib,
  title={cinolib: a generic programming header only C++ library for processing polygonal and polyhedral meshes},
  author={Livesu, Marco},
  journal={Transactions on Computational Science XXXIV},
  series={Lecture Notes in Computer Science},
  pages={64--76},
  year={2019},
  publisher={Springer},
  url={https://github.com/mlivesu/cinolib/},
  doi={10.1007/978-3-662-59958-7_4}
}
@article{livesu2021optimal,
  title={Optimal dual schemes for adaptive grid based hexmeshing},
  author={Marco Livesu and Luca Pitzalis and Gianmarco Cherchi},
  journal={ACM Transactions on Graphics},
  year={2021},
  doi={10.1145/3494456}
}
@inproceedings{marechal2009advances,
  title={Advances in octree-based all-hexahedral mesh generation: handling sharp features},
  author={Maréchal, Loïc},
  booktitle={Proceedings of the 18th international meshing roundtable},
  pages={65--84},
  year={2009},
  organization={Springer},
  doi={10.1007/978-3-642-04319-2_5}
}
@article{marechal2016all,
  title={All hexahedral boundary layers generation},
  author={Maréchal, Loïc},
  journal={Procedia engineering},
  volume={163},
  pages={5--19},
  year={2016},
  publisher={Elsevier},
  doi={10.1016/j.proeng.2016.11.007}
}
@article{pitzalis2021generalized,
  title={Generalized adaptive refinement for grid-based hexahedral meshing},
  author={Pitzalis, Luca and Livesu, Marco and Cherchi, Gianmarco and Gobbetti, Enrico and Scateni, Riccardo},
  journal={ACM Transactions on Graphics (TOG)},
  volume={40},
  number={6},
  pages={1--13},
  year={2021},
  publisher={ACM New York, NY, USA},
  doi={10.1145/3478513.3480508}
}

## Phase 1: Core Implementation (Completed)

- [x] **Initial Data Structures**: Created `Point`, `Rectangle`, `Node`, and `Quadtree` structs to form the basis of the library.
- [x] **Point Insertion**: Implemented an `insert` method to add points to the quadtree.
- [x] **Recursive Refinement**: Implemented a `refine` method that recursively subdivides the tree based on point locations, up to a `level_max`.
- [x] **Code Refactoring & Cleanup**:
    - Separated the concerns of point insertion (`insert`) and tree construction (`refine`).
    - Improved the efficiency of `insert` and `subdivide` methods.
    - Corrected project configuration (`Cargo.toml`) and fixed various bugs.
- [x] **API Documentation**: Added documentation comments to all public structs and functions to improve clarity and enable `cargo doc`.
- [x] **Visualization**: Created a `visualize` method that serializes the tree to YAML and uses an external Python script to generate a PNG image. Made this more robust by using the system's temporary directory.

## Phase 2: Balancing

This phase focuses on implementing standard balancing conditions to ensure the quadtree is well-formed.

### 1. Implement Weak Balancing (Completed)

This feature is now implemented. A `weak_balance` function was added that iteratively subdivides leaves until the 2:1 balance condition is met. The implementation involved adding several helper functions for neighbor finding and tree traversal.

- [x] **Create `weak_balance` function**: Implemented as a post-processing step that operates in a loop.
- [x] **Implement Neighbor Finding**: Implemented a robust, traversal-based `face_neighbors` function. This approach uses recursive helpers (`find_neighbors_recursive` and `get_leaves_on_edge`) to find adjacent leaves by walking the tree's structure, avoiding geometric intersection tests.
- [x] **Implement Balancing Pass**: Created `balance_pass_weakly` which traverses the tree, finds imbalances, and collects nodes for subdivision using raw pointers in a `HashSet`.
- [x] **Subdivide Unbalanced Leaves**: Implemented `subdivide_leaves_by_pointer` to perform the subdivisions identified in the balancing pass.
- [x] **Add Tests**: Created and debugged a unit test (`test_weak_balance`) that manually constructs an unbalanced tree and asserts that it is correctly balanced.
- [x] **Add Visualization**: Updated `main.rs` to visualize the before-and-after state of the tree when `weak_balance` is called, demonstrating the feature.
- [x] **Debug and Refine Balancing**: Iteratively debugged the weak balancing implementation, fixing two subtle bugs related to corner-adjacency being incorrectly treated as face-adjacency. This ensured the balancing algorithm is both correct and robust.
- [x] **Add Regression Test for Corner Adjacency**: Added a new integration test (`test_weak_balance_no_unnecessary_corner_refinement`) that specifically validates the correct balancing behavior for complex corner-adjacent cases, preventing future regressions.

### 2. Implement Primal Meshing Strategy (TODO)

This phase involves implementing a primal meshing scheme by decomposing larger cells that have hanging nodes on their boundaries. The decomposition will follow specific templates, such as the one described in the Sandia SIBL project, to create a high-quality, hex-dominant-style mesh.

- [ ] **Define Hanging Node Templates**: Research and define the specific geometric templates for decomposing cells with 2, 3, and 4 hanging nodes, based on the SIBL "primal" strategy.
- [ ] **Implement Hanging Node Detection**: Create a function to iterate through all leaf cells and, for each, identify the number and location of hanging nodes on its boundary.
- [ ] **Implement Template Application**: Write a function that takes a cell and its hanging node configuration and generates the appropriate internal diagonal lines based on the defined templates.
- [ ] **Add Visualization**: Update the visualization script to draw the generated primal mesh lines.

### 3. Implement Strong Balancing (Next Steps)

A strongly balanced quadtree is stricter: any two leaf nodes that share an edge *or a vertex* cannot differ by more than one level of refinement.

- [ ] **Create `strong_balance` function**:
    - This will follow a similar post-processing pattern to `weak_balance`.
- [ ] **Extend Neighbor Finding**:
    - Create a new or extended neighbor-finding function (`all_neighbors`) that finds leaves sharing either an edge or a vertex.
- [ ] **Implement Strong Balancing Pass**:
    - Adapt the balancing pass logic to use the new neighbor-finding function and enforce the stricter strong-balancing condition.
- [ ] **Add Tests**: Create specific tests for strong balancing, particularly focusing on vertex-adjacent nodes.

### 4. Identify and Pair Hanging Nodes (Next Steps)

This section outlines the algorithm for identifying hanging nodes and processing them to facilitate mesh generation.

#### Stage 1: Identify All "Hanging Edges"

A "hanging edge" is an edge of a cell that has one or more vertices of adjacent, smaller cells lying on it. The goal is to produce a list of these hanging edges.

- **[ ] Create a recursive traversal function.**
  - This function will walk the quadtree and inspect the boundaries between cells (e.g., `find_hanging_edges_recursive`).

- **[ ] Process internal boundaries.**
  - Inside the recursive function, when encountering a `Node::Children`, check the four internal boundaries for differences in refinement level.

- **[ ] Compare refinement levels.**
  - For each boundary, if one adjacent cell is a `Leaf` and the other is `Children`, a hanging edge has been found.

- **[ ] Store the results in a structured way.**
  - When a hanging edge is found, store its properties in a custom struct:
    ```rust
    struct HangingEdge {
        // The two vertices of the original, larger edge
        v1: Point,
        v2: Point,
        // The node(s) that "hang" on this edge
        hanging_nodes: Vec<Point>,
    }
    ```

- **[ ] Recurse to deeper levels.**
  - After checking the internal boundaries of a node, call the recursive function on its four children to find hanging edges at deeper levels.

#### Stage 2: "Pairing" the Hanging Nodes

"Pairing" the hanging nodes can be interpreted as creating new, smaller edges from the hanging edges identified in Stage 1.

- **[ ] Process the `HangingEdge` list.**
  - Iterate through the `Vec<HangingEdge>` generated in Stage 1.

- **[ ] Create the pairs of new edges.**
  - For each `HangingEdge`, use its original vertices (`v1`, `v2`) and the hanging node (`h`) to define two new, smaller edges:
    - **Pair-Part 1:** The edge from `v1` to `h`.
    - **Pair-Part 2:** The edge from `h` to `v2`.