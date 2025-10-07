// mod quadtree;  // Declare the quadtree module
use dirs;
use quadtree::*;
use std::fs; // Bring all public items from quadtree into scope

fn main() -> Result<(), String> {
    println!("Hello quadtree!");

    // Step 0: Setup
    // Create an output directory for output YAML and visualization
    let home_dir =
        dirs::home_dir().ok_or_else(|| "Could not find the home directory".to_string())?;
    let scratch_path = home_dir.join("scratch").join("quadtree");

    // Ensure the custom directory exists
    fs::create_dir_all(&scratch_path).map_err(|e| {
        format!(
            "Failed to create custom directory {:?}: {}",
            scratch_path, e
        )
    })?;
    println!("Using scratch path: {:?}", scratch_path);
    let scratch_path_str = scratch_path.to_string_lossy();

    // Example 1
    println!("----------------------------------------------------");
    println!("Example 1: Quadtree Creation and Levels");
    let boundary = Rectangle {
        origin: Point { x: 1.0, y: -1.0 },
        width: 2.0,
        height: 2.0,
    };

    let mut tree_1 = Quadtree::new(boundary, 0);

    println!("Inserting points...");
    // ne_ne_sw_sw_ne quadrant (up to level 5 refinement)
    tree_1.insert(Point { x: 2.6, y: 0.6 });

    println!("\nFinal Quadtree Structure:");
    println!("{:#?}", tree_1);

    println!("Refining quadtree...");
    tree_1.refine();

    // Visualize the unbalanced tree
    println!("\nVisualizing quadtree BEFORE balancing...");
    if let Err(e) = tree_1.visualize(&scratch_path_str, "example_1_before_balance") {
        eprintln!("Visualization failed: {}", e);
    }

    // Run the weak balancing algorithm
    println!("\nRunning weak_balance()...");
    tree_1.weak_balance();
    println!("Balancing complete.");

    // Visualize the balanced tree
    println!("\nVisualizing quadtree AFTER balancing...");
    if let Err(e) = tree_1.visualize(&scratch_path_str, "example_1_after_balance") {
        eprintln!("Visualization filed: {}", e);
    }

    // Example 2 and 3
    // tree_n.insert(Point { x: 2.1, y: 0.1 });
    // Try to reproduce Figure 5 from
    // Pitzalis_2021_generalized_adaptive_refinement.pdf
    println!("----------------------------------------------------");
    println!("Example 2: Unbalanced and Weakly Balanced Quadtree");
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
    if let Err(e) = tree_2.visualize(&scratch_path_str, "example_2_before_balance") {
        eprintln!("Visualization failed: {}", e);
    }

    // Run the weak balancing algorithm
    println!("\nRunning weak_balance()...");
    tree_2.weak_balance();
    println!("Balancing complete.");

    // Visualize the balanced tree
    println!("\nVisualizing quadtree AFTER balancing...");
    if let Err(e) = tree_2.visualize(&scratch_path_str, "example_2_after_balance") {
        eprintln!("Visualization filed: {}", e);
    }

    Ok(())
}
