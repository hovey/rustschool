// The interface
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Square {
    side_length: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side_length * self.side_length
    }
}

enum Shape {
    Round(Circle),
    Nonround(Square),
}


impl Area for Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Round(x) => x.area(),
            Shape::Nonround(x) => x.area(),
        }
    }
}





fn main() {

    let size: f64 = 10.0;
    let c1 = Circle { radius: size };
    let s1 = Square { side_length: size };

    let shape1 = Shape::Round(c1);
    let shape2 = Shape::Nonround(s1);

    let shapes: Vec<Shape> = vec![shape1, shape2];
    
    let areas: Vec<f64> = shapes.iter().map(|x| x.area()).collect();
    println!("areas: {:?}", areas);
}
