# Remesh - A tool for generating realistic mesh from video

This document outlines the plan for creating `remesh`, a command-line tool that generates a 3D mesh from a video file.

## 1. Project Setup

- [x] Initialize a new Rust project using `cargo new remesh`.
- [x] Create a `README.md` to describe the project.
- [x] Create a `.gitignore` file.
- [x] Create a `PLAN.md` to outline the development plan.

## 2. Core Data Structures

The core of our application will revolve around a few key data structures. These will be defined in `src/main.rs` initially, and later moved to their own modules as the project grows.

### `Vertex`

A `Vertex` represents a point in 3D space.

- `x`: `f32`
- `y`: `f32`
- `z`: `f32`

### `Face`

A `Face` is a triangle that connects three vertices.

- `v1`: `usize` (index of the first vertex)
- `v2`: `usize` (index of the second vertex)
- `v3`: `usize` (index of the third vertex)

### `Mesh`

A `Mesh` represents a 3D object.

- `vertices`: `Vec<Vertex>`
- `faces`: `Vec<Face>`

## 3. Video Processing

This will be the most complex part of the project. We'll need to:

- Read a video file frame by frame.
- For each frame, perform feature detection to identify key points.
- Track these key points across frames to create a 3D point cloud.
- Use the point cloud to generate a mesh.

We'll start by using a placeholder for video processing and focus on the data structures and mesh generation first.

## 4. Mesh Generation

Once we have a point cloud (for now, we'll use a dummy one), we'll need to generate a mesh. We'll use the Delaunay triangulation algorithm for this.

## 5. Command-Line Interface

We'll use the `clap` crate to create a command-line interface for our tool. The CLI will allow users to:

- Specify the input video file.
- Specify the output mesh file.
- Control various parameters of the mesh generation process.

## 6. File I/O

We'll need to be able to:

- Read video files (using a crate like `opencv`).
- Write mesh files (in a standard format like `.obj`).

## 7. Development Plan

1.  **Phase 1: Core Data Structures (Week 1)**
    - [ ] Implement the `Vertex`, `Face`, and `Mesh` structs.
    - [ ] Write unit tests for the data structures.

2.  **Phase 2: Mesh Generation (Week 2)**
    - [ ] Implement a dummy point cloud generator.
    - [ ] Implement the Delaunay triangulation algorithm.
    - [ ] Write unit tests for the mesh generation.

3.  **Phase 3: Video Processing (Weeks 3-4)**
    - [ ] Integrate a video processing library.
    - [ ] Implement feature detection and tracking.
    - [ ] Connect the video processing to the mesh generation.

4.  **Phase 4: CLI and File I/O (Week 5)**
    - [ ] Add `clap` for command-line argument parsing.
    - [ ] Implement file I/O for saving the mesh.

5.  **Phase 5: Refinement and Documentation (Week 6)**
    - [ ] Refactor the code for clarity and performance.
    - [ ] Write comprehensive documentation.