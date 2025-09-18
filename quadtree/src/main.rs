// mod quadtree;  // Declare the quadtree module
use quadtree::*; // Bring all public items from quadtree into scope

fn main() {
    println!("Hello quadtree!");

    let boundary = Rectangle {
        origin: Point { x: 0.0, y: 0.0 },
        width: 100.0,
        height: 100.0,
    };

    let mut quadtree = Quadtree::new(boundary, 0);

    println!("Inserting points...");
    // sw quadrant
    quadtree.insert(Point { x: 10.0, y: 10.0 });
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

    // Call the visualization method
    println!("\nVisualizing quadtree...");
    match quadtree.visualize() {
        Ok(_) => println!("Visualization command sent."),
        Err(e) => eprintln!("Visualization failed: {}", e),
    }
}
