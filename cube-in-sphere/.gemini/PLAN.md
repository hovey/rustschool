# Plan for the project

- [x] Create a mesh.rs for a finite element mesh data structure.  The mesh consists of a collection of nodes in R3.  Each node has a coordinate (x, y, z), which are all float values.  The mesh also consists of elements.  Each element, a linear hexahedral finite element, is a collection of eight non-negative integers, where eaach integer represents a node number. A
- [x] Create a data.rs file that has cube_1 and cube_2, two example mesh files.
- [x] Create a tess.rs for a tesselation.  The tesselation consists of a collection of nodes in R3.  Each node has a coordinate (x, y, z), which are all float values.  The tesselation also consists of elements.  Each element, a linear 3D triangular element, is a collection of three non-negative integers, where each integer represents a node number.
- [ ] Add to data.rs two tesselations, sphere_1 and sphere_2, where sphere_1 is created from the octa_loop00.stl and sphere_2 is created from octa_loop01.stl.  Both of these STL files are in the data folder, which is parallel to the src folder for the project.

### Code Quality Guidelines

- **Avoid Magic Numbers**: Do not use hard-coded literal values in loops or calculations. Instead, define them as local constants, variables, or project-wide constants to improve readability and maintainability.
**Reference Constants**: When a set of values (like coordinates) is used multiple times, define them once (e.g., in an array or constant) and reference that definition throughout the scope.
