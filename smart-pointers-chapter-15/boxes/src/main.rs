fn main() {
    println!("Hello, world!");
    // store an i32 on the heap
    let b = Box::new(5);
    println!("b = {}", b);
}
