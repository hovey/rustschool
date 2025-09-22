// mod quadtree;  // Declare the quadtree module
use quadtree::*; // Bring all public items from quadtree into scope

fn main() {
    println!("Hello quadtree!");

    let boundary = Rectangle {
        origin: Point { x: 1.0, y: -1.0 },
        width: 2.0,
        height: 2.0,
    };

    let mut quadtree = Quadtree::new(boundary, 1);

    println!("Inserting points...");
    // sw quadrant
    quadtree.insert(Point { x: 2.6, y: 0.6 });
    // quadtree.insert(Point { x: 12.0, y: 15.0 });
    // // nw quadrant
    // quadtree.insert(Point { x: 80.0, y: 80.0 });
    // quadtree.insert(Point { x: 90.0, y: 95.0 });
    // // nw quadrant
    // quadtree.insert(Point { x: 40.0, y: 60.0 });
    // // se quadrant
    // quadtree.insert(Point { x: 60.0, y: 40.0 });

    // println!("\nFinal Quadtree Structure:");
    // println!("{:#?}", quadtree);

    println!("Refining quadtree...");
    quadtree.refine();

    // Call the visualization method
    println!("\nVisualizing quadtree...");
    match quadtree.visualize() {
        Ok(_) => println!("Visualization command sent."),
        Err(e) => eprintln!("Visualization failed: {}", e),
    }
}
