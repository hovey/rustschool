# Plan for the Quadtree Project

This plan tracks the development of the Rust Quadtree library.

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
  author={Mar{\'e}chal, Lo{\"\i}c},
  booktitle={Proceedings of the 18th international meshing roundtable},
  pages={65--84},
  year={2009},
  organization={Springer},
  doi={10.1007/978-3-642-04319-2_5}
}
@article{marechal2016all,
  title={All hexahedral boundary layers generation},
  author={Mar{\'e}chal, Lo{\"\i}c},
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

## Phase 2: Balancing (Next Steps)

This phase focuses on implementing standard balancing conditions to ensure the quadtree is well-formed.

### 1. Implement Weak Balancing

A weakly balanced quadtree ensures that adjacent leaf nodes (those sharing a full edge) differ by at most one level of refinement.

- [ ] **Create `weak_balance` function**:
    - This function will be a post-processing step run after `refine`.
    - It will operate in a loop, repeatedly scanning the tree and making modifications until the tree is fully balanced.
- [ ] **Implement Neighbor Finding**:
    - To check the balance condition, we need to find the neighbors of a given leaf node.
    - Create a helper function, likely `face_neighbors`, that can find the leaves sharing an edge (North, South, East, West) with a given leaf. This will probably involve querying the tree from the root based on the leaf's boundary.
- [ ] **Implement Balancing Pass**:
    - Create a helper function `balance_pass_weakly` that traverses the tree.
    - For each leaf, it will use the neighbor-finding function to get its adjacent leaves.
    - If a leaf `L` at level `l` has a neighbor `N` at level `n` where `l > n + 1`, it means `N` must be subdivided to enforce the balance condition.
    - The function will identify all such leaves that need to be subdivided in a single pass.
- [ ] **Subdivide Unbalanced Leaves**:
    - After a pass, the `weak_balance` function will subdivide all the leaves that were marked as needing subdivision.
    - The process repeats until a pass completes with no subdivisions, at which point the tree is weakly balanced.
- [ ] **Add Tests**: Create new unit tests to verify that `weak_balance` correctly balances various unbalanced tree configurations.

### 2. Implement Strong Balancing

A strongly balanced quadtree is stricter: any two leaf nodes that share an edge *or a vertex* cannot differ by more than one level of refinement.

- [ ] **Create `strong_balance` function**:
    - This will follow a similar post-processing pattern to `weak_balance`.
- [ ] **Extend Neighbor Finding**:
    - Create a new or extended neighbor-finding function (`all_neighbors`) that finds leaves sharing either an edge or a vertex.
- [ ] **Implement Strong Balancing Pass**:
    - Adapt the balancing pass logic to use the new neighbor-finding function and enforce the stricter strong-balancing condition.
- [ ] **Add Tests**: Create specific tests for strong balancing, particularly focusing on vertex-adjacent nodes.