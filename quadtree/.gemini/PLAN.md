# Plan for the Quadtree Project

This plan tracks the development of the Rust Quadtree library.

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

## Phase 2: Balancing (Next Steps)

This phase focuses on implementing standard balancing conditions to ensure the quadtree is well-formed.

### 1. Implement Weak Balancing

A weakly balanced quadtree ensures that adjacent leaf nodes (those sharing a full edge) differ by at most one level of refinement.

- [ ] **Create `balance_weakly` function**:
    - This function will be a post-processing step run after `refine`.
    - It will operate in a loop, repeatedly scanning the tree and making modifications until the tree is fully balanced.
- [ ] **Implement Neighbor Finding**:
    - To check the balance condition, we need to find the neighbors of a given leaf node.
    - Create a helper function, likely `find_face_neighbors`, that can find the leaves sharing an edge (North, South, East, West) with a given leaf. This will probably involve querying the tree from the root based on the leaf's boundary.
- [ ] **Implement Balancing Pass**:
    - Create a helper function `balance_pass_weakly` that traverses the tree.
    - For each leaf, it will use the neighbor-finding function to get its adjacent leaves.
    - If a leaf `L` at level `l` has a neighbor `N` at level `n` where `l > n + 1`, it means `N` must be subdivided to enforce the balance condition.
    - The function will identify all such leaves that need to be subdivided in a single pass.
- [ ] **Subdivide Unbalanced Leaves**:
    - After a pass, the `balance_weakly` function will subdivide all the leaves that were marked as needing subdivision.
    - The process repeats until a pass completes with no subdivisions, at which point the tree is weakly balanced.
- [ ] **Add Tests**: Create new unit tests to verify that `balance_weakly` correctly balances various unbalanced tree configurations.

### 2. Implement Strong Balancing

A strongly balanced quadtree is stricter: any two leaf nodes that share an edge *or a vertex* cannot differ by more than one level of refinement.

- [ ] **Create `balance_strongly` function**:
    - This will follow a similar post-processing pattern to `balance_weakly`.
- [ ] **Extend Neighbor Finding**:
    - Create a new or extended neighbor-finding function (`find_all_neighbors`) that finds leaves sharing either an edge or a vertex.
- [ ] **Implement Strong Balancing Pass**:
    - Adapt the balancing pass logic to use the new neighbor-finding function and enforce the stricter strong-balancing condition.
- [ ] **Add Tests**: Create specific tests for strong balancing, particularly focusing on vertex-adjacent nodes.