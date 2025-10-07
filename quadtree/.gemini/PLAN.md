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
  author={Mar{\'e}chal, Lo{\"i}c},
  booktitle={Proceedings of the 18th international meshing roundtable},
  pages={65--84},
  year={2009},
  organization={Springer},
  doi={10.1007/978-3-642-04319-2_5}
}
@article{marechal2016all,
  title={All hexahedral boundary layers generation},
  author={Mar{\'e}chal, Lo{\"i}c},
  journal={Procedia engineering},
  volume={163},
  pages={5--19},
  year={2016},
  publisher={Elsevier},
  doi={10.1016/j.proeng.2016.11.007}
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
- [x] **Implement Neighbor Finding**: Created `face_neighbors` and a recursive helper `find_leaves_in_bounds` to find adjacent leaves. Added an `intersects` method to `Rectangle` to support this.
- [x] **Implement Balancing Pass**: Created `balance_pass_weakly` which traverses the tree, finds imbalances, and collects nodes for subdivision using raw pointers in a `HashSet`.
- [x] **Subdivide Unbalanced Leaves**: Implemented `subdivide_leaves_by_pointer` to perform the subdivisions identified in the balancing pass.
- [x] **Add Tests**: Created and debugged a unit test (`test_weak_balance`) that manually constructs an unbalanced tree and asserts that it is correctly balanced.
- [x] **Add Visualization**: Updated `main.rs` to visualize the before-and-after state of the tree when `weak_balance` is called, demonstrating the feature.

### 2. Implement Strong Balancing (Next Steps)

A strongly balanced quadtree is stricter: any two leaf nodes that share an edge *or a vertex* cannot differ by more than one level of refinement.

- [ ] **Create `strong_balance` function**:
    - This will follow a similar post-processing pattern to `weak_balance`.
- [ ] **Extend Neighbor Finding**:
    - Create a new or extended neighbor-finding function (`all_neighbors`) that finds leaves sharing either an edge or a vertex.
- [ ] **Implement Strong Balancing Pass**:
    - Adapt the balancing pass logic to use the new neighbor-finding function and enforce the stricter strong-balancing condition.
- [ ] **Add Tests**: Create specific tests for strong balancing, particularly focusing on vertex-adjacent nodes.
