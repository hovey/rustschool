#[derive(Debug)]
struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Square { side }
    }
}

#[derive(Debug)]
struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
}

impl Triangle {
    pub fn new(side1: f64, side2: f64, side3: f64) -> Self {
        Triangle { side1, side2, side3 }
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

pub trait Perimeter {
    fn perimeter(&self) -> f64;
}

impl Perimeter for Square {
    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

impl Perimeter for Triangle {
    fn perimeter(&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }
}

impl Perimeter for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// // A generic utility function that accepts any type that implements the Perimeter trait
// fn print_perimeter<T: Perimeter>(shape: &T) {
//     println!("The perimeter is: {:.2}", shape.perimeter());
// }

// Update the print_perimeter to accept a trait object reference so we can use it
// in a loop without excessive boilerplate
fn print_perimeter(shape: &dyn Perimeter) {
    println!("The perimeter is {:.2}", shape.perimeter());
}

fn main() {
    let sq = Square::new(10.0);
    let tr = Triangle::new(3.0, 4.0, 5.0);
    let cr = Circle::new(7.5);

    // println!("Square side: {}", sq.side);
    // println!("Triangle sides: {}, {}, {}", tr.side1, tr.side2, tr.side3);
    // println!("Circle radius: {}", cr.radius);

    // Use dynamic dispatch (polymorphism) using trait objects
    // Create a vecdtor of trait objects (Box<dyn Perimeter>)
    // Box<dyn Trait> is used because all objects must be behind a pointer
    // (like Box) since they don't have a known size at compile time.
    let shapes: Vec<Box<dyn Perimeter>> = vec![
        Box::new(sq),
        Box::new(tr),
        Box::new(cr),
    ];

    // Iterate over the vector and call the print_perimeter function
    for shape in shapes.iter() {
        // The dereference happens automatically when calling the method
        // through the trait object, enabling dynamic dispatch.
        
        // The double dereference &**shape is necessary because of how Rust
        // handles references, boxing, and trait objects together:
        // 1. The shapes.iter() returns an iterator over references to the items
        // in the vector.  Since the vector contains Box<dyn Perimeter>, the type
        // of shape in the loop is
        // shape: &Box<dyn Perimeter>
        // 2. First dereference (*shape): Applying the first dereference moves
        // from the referencee to the Box
        // to the Box itself:
        // *shape: Box<dyn Perimeter>
        // 3. Second dereference (**shape): Aplyign the second dereference relies
        // on the Defer trait implementation for Box.  Box<T> implements Deref to
        // return a reference to the inner value T.  In this case, T is dyn Perimeter:
        // **shape: dyn Perimeter
        // The actual trait object is on the heap, holding the data and the vtable
        // (virtual method table) pointer.
        // 4. Final reference (&**shape): The print_perimeter function is defined
        // to take a reference.  Therefore, you need to take a final reference trait
        // object
        let shape_ref: &dyn Perimeter = &**shape;

        // While &**shape is technically correct and explicit, a key feature in Rust
        // is Deref Coercion.  Because Box<dyn Perimeter> implements Deref, Rust can
        // automatically perform the necessary conversions to match a function
        // signature.

        // We can slightly simplify the call by passing a reference to the Boxed shape
        // an using the original function definition if we change it slightly:
        print_perimeter(shape_ref);

    }
    // The simplish way to iterate and call a method on the trait object is to
    // call the method directly on the Box reference, relying on coercion
    // Below is the most idiomatic simplification because it lets the compiler handle
    // the reference and dereference details necessary for dynamic dispatch.
    println!("More idiomatic...");
    for shape_box in shapes.iter() {
        println!("The perimeter is {:.2}", shape_box.perimeter());
    }

}
