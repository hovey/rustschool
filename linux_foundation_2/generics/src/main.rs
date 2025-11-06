// The sum fuction is not strictly needed if you use the built-in .sum() method,
// but if you want to wrap itin a function for practice:
// fn sum<T>(items: &Vec<T>) -> T
fn sum<T>(items: &[T]) -> T
where
    // T must implement the Clone trait so we can create a copy of the initial zero
    // default value
    T: std::clone::Clone,
    // T must implement the Sum trait, which handles the necessary Add and Default
    // bounds internally.
    T: std::iter::Sum,
    {
        // The .iter() provides references to the elements (&T).
        // The .cloned() converts &T into T (requiring Clone).
        // The .sum() accumulates the values (requiring Sum).
        items.iter().cloned().sum()
    }

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![1.1, 2.2, 3.3, 4.4, 5.5];

    println!("{}", sum(&v1));
    println!("{}", sum(&v2));
}
