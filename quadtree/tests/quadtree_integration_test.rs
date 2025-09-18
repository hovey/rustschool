use quadtree::*;  // Import everything from quadtree

#[test]
fn integration_test_quadtree_subdivision_and_reinsertion() {
    let boundary = Rectangle {
        origin: Point { x: 0.0, y: 0.0 },
        width: 100.0,
        height: 100.0,
    };
    let mut quadtree = Quadtree::new(boundary, 1);  // max_levels is 1
    let point = Point { x: 50.0, y: 60.0 };  // poin in the ne quadrant

    assert!(quadtree.insert(point.clone()));
}