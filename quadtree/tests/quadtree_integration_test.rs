use quadtree::*; // Import everything from quadtree

#[test]
fn integration_test_quadtree_subdivision_and_reinsertion() {
    let boundary = Rectangle {
        origin: Point { x: 0.0, y: 0.0 },
        width: 100.0,
        height: 100.0,
    };
    let mut quadtree = Quadtree::new(boundary, 1); // level_max is 1
    let point = Point { x: 50.0, y: 60.0 }; // poin in the ne quadrant

    assert!(quadtree.insert(point.clone()));
}

#[test]
fn test_weak_balance_no_unnecessary_corner_refinement() {
    // This test reproduces the scenario from Example 2 in main.rs, which was
    // causing incorrect refinements due to corner adjacencies.
    let mut tree = Quadtree::new(
        Rectangle {
            origin: Point { x: 0.0, y: 0.0 },
            width: 4.0,
            height: 4.0,
        },
        5, // level_max
    );

    // 1. Create a deep refinement in the NE -> SW -> SW quadrant.
    tree.subdivide(); // L1
    let ne_l1 = match &mut tree.node {
        Node::Children { ne, .. } => ne,
        _ => panic!("Tree should have L1 children"),
    };
    ne_l1.subdivide(); // L2
    let ne_sw_l2 = match &mut ne_l1.node {
        Node::Children { sw, .. } => sw,
        _ => panic!("NE child should have L2 children"),
    };
    ne_sw_l2.subdivide(); // L3
    let ne_sw_sw_l3 = match &mut ne_sw_l2.node {
        Node::Children { sw, .. } => sw,
        _ => panic!("NE->SW child should have L3 children"),
    };
    ne_sw_sw_l3.subdivide(); // L4

    // 2. Run the balancing algorithm.
    tree.weak_balance();

    // 3. Assert that corner-adjacent leaves were not unnecessarily refined.
    // The `nw` and `se` quadrants are correctly refined once (from L1 to L2)
    // to balance against their neighbors in the `ne` quadrant.
    // The bug was that children of `nw` and `se` (e.g., `nw_ne`) were being
    // refined a second time due to a corner-adjacency bug.
    match &tree.node {
        Node::Children { nw, se, .. } => {
            // Check the nw quadrant's children
            match &nw.node {
                Node::Children { nw: _nw_nw, ne: nw_ne, sw: _nw_sw, se: nw_se } => {
                    // nw_ne is not face-adjacent to the deepest refinement. It should remain a leaf.
                    assert!(
                        matches!(nw_ne.node, Node::Leaf { .. }),
                        "nw_ne should remain a leaf"
                    );

                    // nw_se IS face-adjacent to the L4 refinement area and MUST be refined.
                    assert!(
                        matches!(nw_se.node, Node::Children { .. }),
                        "nw_se should be refined"
                    );
                }
                _ => panic!("nw quadrant should have been subdivided once."),
            }

            // Check the se quadrant's children
            match &se.node {
                Node::Children { nw: se_nw, ne: se_ne, sw: _se_sw, se: _se_se } => {
                    // se_ne is not face-adjacent to the deepest refinement. It should remain a leaf.
                    assert!(
                        matches!(se_ne.node, Node::Leaf { .. }),
                        "se_ne should remain a leaf"
                    );

                    // se_nw IS face-adjacent to the L4 refinement area and MUST be refined.
                    assert!(
                        matches!(se_nw.node, Node::Children { .. }),
                        "se_nw should be refined"
                    );
                }
                _ => panic!("se quadrant should have been subdivided once."),
            }
        }
        _ => panic!("Tree should be a Children node."),
    }
}