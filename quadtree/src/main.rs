// mod quadtree;  // Declare the quadtree module
use dirs;
use quadtree::*;
use std::f64::consts::PI;
use std::fs; // Bring all public items from quadtree into scope

fn circle_points(num_points: u32) -> Vec<Point> {
    let radius: f64 = 1.0;
    let mut points: Vec<Point> = Vec::new();

    // The full circle with 2 * PI radians.
    let angle_step: f64 = 2.0 * PI / (num_points as f64);

    for i in 0..num_points {
        let angle = i as f64 * angle_step;

        let x: f64= radius * angle.cos();
        let y: f64 = radius * angle.sin();

        points.push(Point { x, y });
    }
    points
}

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
    // Create a L0 quadtree with a maximum of five refinement levels
    let mut tree_1 = Quadtree::new(
        Rectangle {
            origin: Point { x: 1.0, y: -1.0 },
            width: 2.0,
            height: 2.0,
        },
        5,
    );

    println!("Inserting points...");
    // ne_ne_sw_sw_ne quadrant (up to level 5 refinement)
    tree_1.insert(Point { x: 2.6, y: 0.6 });

    // println!("\nQuadtree before refinement:");
    // println!("{:#?}", tree_1);

    println!("\nRefining quadtree...");
    tree_1.refine();

    // println!("\nQuadtree after refinement:");
    // println!("{:#?}", tree_1);

    // Visualize the unbalanced tree
    println!("\nVisualizing quadtree BEFORE balancing...");
    if let Err(e) = tree_1.visualize(&scratch_path_str, "example_1_before_balancing") {
        eprintln!("Visualization failed: {}", e);
    }

    // Run the weak balancing algorithm
    println!("\nRunning weak_balance()...");
    tree_1.weak_balance();
    println!("Balancing complete.");

    // Visualize the balanced tree
    println!("\nVisualizing quadtree AFTER balancing...");
    if let Err(e) = tree_1.visualize(&scratch_path_str, "example_1_weakly_balanced") {
        eprintln!("Visualization filed: {}", e);
    }

    // Example 2
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

    // subdivide four times to create an unbalanced quadtree
    println!("Creating an unbalanced tree...");

    tree_2.subdivide(); // L0 -> L1
                        // Get the NE child
    let ne = match &mut tree_2.node {
        Node::Children { ne, .. } => ne,
        _ => panic!("L1 NE child should exist."),
    };

    ne.subdivide(); // L1 -> L2
                    // Get the NE_SW child
    let ne_sw = match &mut ne.node {
        Node::Children { sw, .. } => sw,
        _ => panic!("L2 NE_SW child exist."),
    };

    ne_sw.subdivide(); // L2 -> L3
                       // Get the NE_SW_SW child
    let ne_sw_sw = match &mut ne_sw.node {
        Node::Children { sw, .. } => sw,
        _ => panic!("L3 NW_SW_SW child should exist."),
    };

    ne_sw_sw.subdivide(); // L3 -> L4

    println!("Unbalanced tree created.");

    // Visualize the unbalanced tree
    println!("\nVisualizing quadtree BEFORE balancing...");
    if let Err(e) = tree_2.visualize(&scratch_path_str, "example_2_before_balancing") {
        eprintln!("Visualization failed: {}", e);
    }

    // Run the weak balancing algorithm
    println!("\nRunning weak_balance()...");
    tree_2.weak_balance();
    println!("Balancing complete.");

    // Visualize the balanced tree
    println!("\nVisualizing quadtree AFTER balancing...");
    if let Err(e) = tree_2.visualize(&scratch_path_str, "example_2_weakly_balanced") {
        eprintln!("Visualization filed: {}", e);
    }

    // Example 3
    println!("----------------------------------------------------");
    println!("Example 3: Quadtree for a Circle");
    // Create a L0 quadtree with a maximum of five refinement levels
    let mut tree_3 = Quadtree::new(
        Rectangle {
            origin: Point { x: -1.0, y: -1.0 },
            width: 2.0,
            height: 2.0,
        },
        5,
    );

    println!("Inserting points...");
    let circle_points = circle_points(100);
    for point in circle_points {
        // println!("Inserting point: {:?}", point);
        tree_3.insert(point);
    }

    // println!("\nQuadtree before refinement:");
    // println!("{:#?}", tree_3);

    println!("\nRefining quadtree...");
    tree_3.refine();

    // println!("\nQuadtree after refinement:");
    // println!("{:#?}", tree_3);

    // Visualize the unbalanced tree
    println!("\nVisualizing quadtree BEFORE balancing...");
    if let Err(e) = tree_3.visualize(&scratch_path_str, "example_3_before_balancing") {
        eprintln!("Visualization failed: {}", e);
    }

    // Run the weak balancing algorithm
    println!("\nRunning weak_balance()...");
    tree_3.weak_balance();
    println!("Balancing complete.");

    // Visualize the balanced tree
    println!("\nVisualizing quadtree AFTER balancing...");
    if let Err(e) = tree_3.visualize(&scratch_path_str, "example_3_weakly_balanced") {
        eprintln!("Visualization filed: {}", e);
    }

    Ok(())
}
