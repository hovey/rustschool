// mod quadtree;  // Declare the quadtree module
use quadtree::*; // Bring all public items from quadtree into scope

fn main() {
    println!("Hello quadtree!");

    // Example 1
    // Try to reproduce Figure 5 from
    // Pitzalis_2021_generalized_adaptive_refinement.pdf
    let boundary = Rectangle {
        origin: Point { x: 1.0, y: -1.0 },
        width: 2.0,
        height: 2.0,
    };

    let mut tree_1 = Quadtree::new(boundary, 4);

    println!("Inserting points...");
    // ne_sw_sw quadrant
    // tree_1.insert(Point { x: 2.6, y: 0.6 });
    tree_1.insert(Point { x: 2.1, y: 0.1 });

    // println!("\nFinal Quadtree Structure:");
    // println!("{:#?}", tree_1);

    println!("Refining quadtree...");
    tree_1.refine();

    // Visualize the unbalanced tree
    println!("\nVisualizing quadtree BEFORE balancing...");
    if let Err(e) = tree_1.visualize("example_1_before_balance") {
        eprintln!("Visualization failed: {}", e);
    }

    // Run the weak balancing algorithm
    println!("\nRunning weak_balance()...");
    tree_1.weak_balance();
    println!("Balancing complete.");

    // Visualize the balanced tree
    println!("\nVisualizing quadtree AFTER balancing...");
    if let Err(e) = tree_1.visualize("example_1_after_balance") {
        eprintln!("Visualization filed: {}", e);
    }

    // Example 2
    println!("\nExample 2: Unbalanced and Weakly Balanced Quadtree");
    let mut tree_2 = Quadtree::new(
        Rectangle {
            origin: Point { x: 0.0, y: 0.0 },
            width: 4.0,
            height: 4.0,
        },
        4, // level_max
    );

    // subdivide three times to create an imbalance
    println!("Creating an unbalanced tree...");
    tree_2.subdivide(); // L1
    let ne_l1 = match &mut tree_2.node {
        Node::Children { ne, .. } => ne,
        _ => panic!("Tree should have children after subdivide"),
    };
    ne_l1.subdivide(); // L2
    let ne_sw_l2 = match &mut ne_l1.node {
        Node::Children { sw, .. } => sw,
        _ => panic!("NE child should have L2 children"),
    };
    ne_sw_l2.subdivide(); // L3
    println!("Unbalanced tree created.");

    // Visualize the unbalanced tree
    println!("\nVisualizing quadtree BEFORE balancing...");
    if let Err(e) = tree_2.visualize("example_2_before_balance") {
        eprintln!("Visualization failed: {}", e);
    }

    // Run the weak balancing algorithm
    println!("\nRunning weak_balance()...");
    tree_2.weak_balance();
    println!("Balancing complete.");

    // Visualize the balanced tree
    println!("\nVisualizing quadtree AFTER balancing...");
    if let Err(e) = tree_2.visualize("example_2_after_balance") {
        eprintln!("Visualization filed: {}", e);
    }
}
