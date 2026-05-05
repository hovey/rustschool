pub mod data;
pub mod mesh;
pub mod tess;

fn main() {
    let c1 = data::cube_1();
    let c2 = data::cube_2();

    let s1 = data::sphere_1();
    let s2 = data::sphere_2();

    println!("Mesh 1 has {} nodes, {} elements.", c1.nodes.len(), c1.elements.len());
    println!("Mesh 2 has {} nodes, {} elements.", c2.nodes.len(), c2.elements.len());

    println!("Sphere 1 has {} nodes, {} elements.", s1.nodes.len(), s1.elements.len());
    println!("Sphere 2 has {} nodes, {} elements.", s2.nodes.len(), s2.elements.len());
}
