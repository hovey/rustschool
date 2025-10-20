use dirs;
use quadtree::*; // Bring all public items from quadtree into scope
use std::f64::consts::PI;
use std::fs;

fn circle_points(num_points: u32) -> Vec<Point> {
    let radius: f64 = 1.0;
    let mut points: Vec<Point> = Vec::new();

    // The full circle with 2 * PI radians.
    let angle_step: f64 = 2.0 * PI / (num_points as f64);

    for i in 0..num_points {
        let angle = i as f64 * angle_step;

        let x: f64 = radius * angle.cos();
        let y: f64 = radius * angle.sin();

        points.push(Point { x, y });
    }
    points
}

fn point_stimulated_refinement(scratch_path_str: &str) -> Result<(), String> {
    // Example 1
    println!("----------------------------------------------------");
    println!("Example 1: Quadtree Creation and Levels");
    // Create a L0 quadtree with a maximum of five refinement levels
    let mut tree = Quadtree::new(
        Rectangle {
            origin: Point { x: 1.0, y: -1.0 },
            width: 2.0,
            height: 2.0,
        },
        5,
    );

    println!("Inserting points...");
    // ne_ne_sw_sw_ne quadrant (up to level 5 refinement)
    tree.insert(Point { x: 2.6, y: 0.6 });

    // println!("\nQuadtree before refinement:");
    // println!("{:#?}", tree);

    println!("\nRefining quadtree...");
    tree.refine();

    // println!("\nQuadtree after refinement:");
    // println!("{:#?}", tree);

    // Visualize the unbalanced tree
    println!("\nVisualizing quadtree BEFORE balancing...");
    if let Err(e) = tree.visualize(&scratch_path_str, "example_1_before_balancing", false) {
        eprintln!("Visualization failed: {}", e);
    }

    // Run the weak balancing algorithm
    println!("\nRunning weak_balance()...");
    tree.weak_balance();
    println!("Balancing complete.");

    // Visualize the balanced tree
    println!("\nVisualizing quadtree AFTER balancing...");
    if let Err(e) = tree.visualize(&scratch_path_str, "example_1_weakly_balanced", false) {
        eprintln!("Visualization failed: {}", e);
    }
    Ok(())
}

fn manual_subdivision(scratch_path_str: &str) -> Result<(), String> {
    // Example 2
    // tree_n.insert(Point { x: 2.1, y: 0.1 });
    // Try to reproduce Figure 5 from
    // Pitzalis_2021_generalized_adaptive_refinement.pdf
    println!("----------------------------------------------------");
    println!("Example 2: Unbalanced and Weakly Balanced Quadtree");
    let mut tree = Quadtree::new(
        Rectangle {
            origin: Point { x: 0.0, y: 0.0 },
            width: 4.0,
            height: 4.0,
        },
        4, // level_max
    );

    // subdivide four times to create an unbalanced quadtree
    println!("Creating an unbalanced tree...");

    tree.subdivide(); // L0 -> L1
                      // Get the NE child
    let ne = match &mut tree.node {
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
    if let Err(e) = tree.visualize(&scratch_path_str, "example_2_before_balancing", false) {
        eprintln!("Visualization failed: {}", e);
    }

    // Run the weak balancing algorithm
    println!("\nRunning weak_balance()...");
    tree.weak_balance();
    println!("Balancing complete.");

    // Visualize the balanced tree
    println!("\nVisualizing quadtree AFTER balancing...");
    if let Err(e) = tree.visualize(&scratch_path_str, "example_2_weakly_balanced", false) {
        eprintln!("Visualization failed: {}", e);
    }

    Ok(())
}

fn circle_with_balancing(scratch_path_str: &str) -> Result<(), String> {
    // Example 3
    println!("----------------------------------------------------");
    println!("Example 3: Quadtree for a Circle");
    // Create a L0 quadtree with a maximum of five refinement levels
    let mut tree = Quadtree::new(
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
        tree.insert(point);
    }

    // println!("\nQuadtree before refinement:");
    // println!("{:#?}", tree);

    println!("\nRefining quadtree...");
    tree.refine();

    // println!("\nQuadtree after refinement:");
    // println!("{:#?}", tree);

    // Visualize the unbalanced tree
    println!("\nVisualizing quadtree BEFORE balancing...");
    if let Err(e) = tree.visualize(&scratch_path_str, "example_3_before_balancing", false) {
        eprintln!("Visualization failed: {}", e);
    }

    // Run the weak balancing algorithm
    println!("\nRunning weak_balance()...");
    tree.weak_balance();
    println!("Balancing complete.");

    // Visualize the balanced tree
    println!("\nVisualizing quadtree AFTER balancing...");
    if let Err(e) = tree.visualize(&scratch_path_str, "example_3_weakly_balanced", false) {
        eprintln!("Visualization failed: {}", e);
    }

    Ok(())
}

fn level_1_fully_refined(scratch_path_str: &str) -> Result<(), String> {
    println!("----------------------------------------------------");
    let title: &str = "level_1_fully_refined";
    println!("Example: {}", title);

    let mut tree: Quadtree = Quadtree::new(
        Rectangle {
            origin: Point { x: -1.0, y: -1.0 },
            width: 2.0,
            height: 2.0,
        },
        2,
    );

    tree.subdivide(); // L0 -> L1

    println!("\nVisualizing quadtree...");
    if let Err(e) = tree.visualize(&scratch_path_str, title, true) {
        eprintln!("Visualization failed: {}", e);
    }

    Ok(())
}

fn transition_wine_glass(scratch_path_str: &str) -> Result<(), String> {
    println!("----------------------------------------------------");
    let title: &str = "template_wine_glass";
    println!("Example: {}", title);

    let mut tree: Quadtree = Quadtree::new(
        Rectangle {
            origin: Point { x: -1.0, y: -1.0 },
            width: 2.0,
            height: 2.0,
        },
        2,
    );

    tree.subdivide(); // L0 -> L1, then get the NE and SE children

    let (ne, se) = match &mut tree.node {
        Node::Children { ne, se, .. } => (ne, se),
        _ => panic!("L1 NE and SE children should exist after subdivision."),
    };

    ne.subdivide(); // L1 -> L2
    se.subdivide(); // L1 -> L2

    println!("\nVisualizing quadtree...");
    if let Err(e) = tree.visualize(&scratch_path_str, title, true) {
        eprintln!("Visualization failed: {}", e);
    }

    Ok(())
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

    // Examples library and selection
    let examples: &[(fn(&str) -> Result<(), String>, bool)] = &[
        (point_stimulated_refinement, false),
        (manual_subdivision, false),
        (circle_with_balancing, false),
        (level_1_fully_refined, true),
        (transition_wine_glass, true),
    ];

    for (func, enabled) in examples {
        if *enabled {
            func(&scratch_path_str)?;
        }
    }

    Ok(())
}
