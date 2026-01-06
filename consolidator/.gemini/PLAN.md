# Plan for the Consolidator Project

This plan tracks the development of the Consolidator functionality.

## Note on Collaboration

I will make the code updates manually.  Please provide your sugestions, and I will apply them.

## Context

We are working on the feature described here: https://github.com/autotwin/automesh/issues/552?issue=autotwin%7Cautomesh%7C546

We are particularly interested in "projecting the boundary of an all-hex dualized mesh onto a target geometry composed of triangular, 3D facets.

We want to explore mesh padding as a way to improve mesh topology, enuring that not element becomes degenerate during the projection process.

Use any useful literature and scholarly documents on quadtree definitions, formulation, and implementation, including the following:

@inproceedings{cherchi2019selective,
  title={Selective padding for polycube-based hexahedral meshing},
  author={Cherchi, Gianmarco and Alliez, Pierre and Scateni, Riccardo and Lyon, Max and Bommes, David},
  booktitle={Computer graphics forum},
  volume={38},
  number={1},
  pages={580--591},
  year={2019},
  url={https://inria.hal.science/hal-01970790/document},
  organization={Wiley Online Library}
}
@inproceedings{gao2019feature,
  title={Feature preserving octree-based hexahedral meshing},
  author={Gao, Xifeng and Shen, Hanxiao and Panozzo, Daniele},
  booktitle={Computer graphics forum},
  volume={38},
  number={5},
  pages={135--149},
  year={2019},
  url={ https://doi.org/10.1111/cgf.13795},
  organization={Wiley Online Library}
}
@article{lin2015quality,
  title={Quality guaranteed all-hex mesh generation by a constrained volume iterative fitting algorithm},
  author={Lin, Hongwei and Jin, Sinan and Liao, Hongwei and Jian, Qun},
  journal={Computer-Aided Design},
  volume={67},
  pages={107--117},
  year={2015},
  url={https://doi.org/10.1016/j.cad.2015.05.004},
  publisher={Elsevier}
}
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

## Development Status

- [ ] **Phase 0: Initial Setup & Data Structures**
- [ ] **Phase 1: Padding (Adding Raw Material)**
  - [ ] Step 1.1: Identify Padding Layer Locations
  - [ ] Step 1.2: Generate Padded Mesh
  - [ ] Step 1.3: (Optional) Iterate Padding for Multiple Layers
- [ ] **Phase 2: Projection (Conforming to the Sphere)**
  - [ ] Step 2.1: Identify Outer Surface Vertices
  - [ ] Step 2.2: Project Vertices to Sphere
- [ ] **Phase 3: Smoothing (Improving Cell Quality)**
  - [ ] Step 3.1: Identify Smoothable Vertices
  - [ ] Step 3.2: Apply Laplacian Smoothing

## Detailed Workflow: Pad -> Project -> Smooth

This section outlines the complete process to achieve a hex mesh that conforms to the spherical boundary.

### Phase 0: Core Data Structures

First, we need to define the core data structures to represent our 3D geometry in `src/main.rs`.

*   `Point3D`: A struct with public `x`, `y`, and `z` fields of type `f64`. It should derive `Copy`, `Clone`, and `Debug`.
*   `Hexahedron`: A struct representing a hex element with a field `vertices` which is an array of 8 `usize` values, corresponding to indices in a vertex list.
*   `Triangle`: A struct representing a triangular facet with a field `vertices` which is an array of 3 `usize` values, corresponding to indices in a vertex list.
*   `HexMesh`: A struct to hold the hexahedral mesh data, containing two public fields:
    *   `vertices: Vec<Point3D>`
    *   `cells: Vec<Hexahedron>`
*   `TriangularMesh`: A struct to hold the triangular boundary mesh data, containing two public fields:
    *   `vertices: Vec<Point3D>`
    *   `facets: Vec<Triangle>`

### Phase 1: Padding (Adding Raw Material)

This phase focuses on adding one or more layers of new grid-aligned hexahedral cells around the existing mesh to create a buffer zone.

*   **Step 1.1: Identify First Padding Layer Locations:** Implement a function `find_padding_layer_locations(existing_mesh: &HexMesh) -> Vec<(i32, i32, i32)>`. This function will:
    *   **Assumption:** The input `HexMesh` is aligned with a regular grid. We need a way to get integer grid coordinates for each cell.
    *   Create a `HashSet<(i32, i32, i32)>` representing the integer grid coordinates occupied by the `existing_mesh`'s cells.
    *   For each occupied cell, check its 6 direct (face-sharing) neighbors on the grid.
    *   Collect all unique, unoccupied neighbor coordinates. These are the locations for the first padding layer.
*   **Step 1.2: Generate Padded Mesh:** Implement a function `create_padded_mesh(base_mesh: &HexMesh, padding_locations: Vec<(i32, i32, i32)>) -> HexMesh`. This function will:
    *   Combine the cells of the `base_mesh` with new cells created at `padding_locations`.
    *   Use a `HashMap` to manage and de-duplicate vertices from both the base mesh and new padding cells, ensuring perfect stitching.
*   **Step 1.3 (Optional but Recommended): Iterate Padding:** The `find_padding_layer_locations` and `create_padded_mesh` functions can be called multiple times. Each iteration adds another layer. Typically, 2-3 layers provide good results for smoothing.

### Phase 2: Projection (Conforming to the Sphere)

This phase modifies the outermost layer of the padded mesh to precisely match the target spherical boundary.

*   **Step 2.1: Identify Outer Surface Vertices:** Implement a function `find_outer_surface_vertices(padded_mesh: &HexMesh) -> HashSet<usize>`. This function will:
    *   Find all faces on the exterior of the *padded* mesh (faces belonging to only one cell). **Implementation Note:** This can be done by building a `HashMap` mapping faces (represented by a sorted tuple of vertex indices) to their parent cell counts. Exterior faces will have a count of 1.
    *   Collect all unique vertex indices belonging to these exterior faces.
*   **Step 2.2: Project Vertices to Sphere:** Implement a function `project_vertices_to_sphere(mesh: &mut HexMesh, vertex_indices: &HashSet<usize>, sphere_radius: f64, sphere_center: Point3D)`. This function will:
    *   For each `vertex_index` in `vertex_indices`:
        *   Calculate the vector from the `sphere_center` to the vertex.
        *   Normalize this vector.
        *   Scale the normalized vector by `sphere_radius`.
        *   Update the vertex's `Point3D` coordinates in the `mesh.vertices` list.
    *   Crucially: only the identified outer surface vertices are moved. All interior vertices (including those from the original mesh) remain fixed on the grid.

### Phase 3: Smoothing (Improving Cell Quality)

This phase iteratively refines the positions of the internal vertices within the padding layers to improve cell shape and reduce distortion introduced by projection.

**Data Management:** To correctly implement smoothing, we must track several sets of vertex indices:
*   `original_vertices`: All vertices from the initial, unpadded mesh.
*   `projected_vertices`: The outer vertices that were moved to the sphere surface in Phase 2.
*   `fixed_vertices`: The union of `original_vertices` and `projected_vertices`. These must not move during smoothing.
*   `smoothable_vertices`: Vertices in the padding layers that are *not* in `fixed_vertices`.

*   **Step 3.1: Identify Smoothable Vertices:** Implement a function `find_smoothable_vertices(padded_mesh: &HexMesh, original_vertices: &HashSet<usize>, projected_vertices: &HashSet<usize>) -> Vec<usize>`.
    *   This function will iterate through all vertices in the `padded_mesh` and collect those that are not in `original_vertices` or `projected_vertices`.
*   **Step 3.2: Apply Laplacian Smoothing:** Implement a function `apply_laplacian_smoothing(mesh: &mut HexMesh, smoothable_vertices: &[usize], fixed_vertices: &HashSet<usize>, iterations: u32)`. This function will:
    *   **Implementation Note:** Requires building a vertex-to-vertex adjacency list (e.g., `HashMap<usize, Vec<usize>>`).
    *   For a specified number of `iterations`:
        *   For each `vertex_id` in `smoothable_vertices`:
            *   Calculate the centroid (average position) of its direct topological neighbors.
            *   Move the vertex's position towards this centroid (e.g., `new_pos = old_pos * 0.5 + centroid * 0.5`).
        *   Ensure `fixed_vertices` are never moved.