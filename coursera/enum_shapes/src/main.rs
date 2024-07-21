#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
}


fn main() {

    println!("Hello, world!");
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0)];

    let cumulative_area: f64 = shapes.iter().map(|shape| match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
    })
    .sum();

    println!("Cumulative area: {}", cumulative_area);

    // result = 87.53981633974483
    assert!(cumulative_area > 87.53981633974482);
    assert!(cumulative_area < 87.53981633974484);
}
