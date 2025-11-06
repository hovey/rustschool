fn main() {
    // Original data: A vector of scores
    let mut scores = vec![10, 20, 30, 40];
    println!("Initial data: {:?}", scores);

    // Use case 1: read-only iterations with .iter()
    // Calculate the sum without modifying or consuming the vector.
    // .iter() yields immutable references (&i32)
    let total: i32 = scores.iter().sum();
    println!("\n1. Read only iteration (total score): {}", total);
    println!("Original vector is still available: {:?}", scores);

    // Use case 2: Mutable iterations with .iter_mut()
    // Modify each element in place (e.g., scale all the scores by 2).
    // .iter_mut() yields mutable references (&mut i32).
    println!("\n2. Mutable iteration (doubling scores):");
    for item in scores.iter_mut() {
        *item *= 2;
    }
    println!("Modified vector {:?}", scores); // [20, 40, 60, 80]

    // Use case 2b: Conditional mutable iteration (filter + modify)
    // Only aply a bonus (+5) to scores that are currently less than 80.
    
    // The syntax .filter.map() is used when you want to transform an item into a
    // new item and collect the result into a new collection (a consuming operation).

    // Because here we want to modify in place using iter_mut(), you must
    // use the interator's consumer method, usually .for_each() to execute the
    // mutation.

    println!("\n2b. Conditional mutable iteration (filter + for_each)");
    scores.iter_mut() // start with mutable references (&mut i32)
        // filter need to check the value.  The closure receives a reference to the
        // element (which is already a mutable reference), so we use **score_ref to
        // get the 832 value
        .filter(|score_ref| **score_ref < 80)
        // for_each consumes the filtered iterator and performs the side effect (mutation)
        .for_each(|score_ref| {
            // Here, score_ref is an &mut i32, we dereference it to modify the value
            *score_ref +=5;
        });
    println!("Bonused vector {:?}", scores); // [25, 45, 65, 80]

    // Use case 3: Consuming iteration with .into_iter()
    // Convert scores into strings, consuming the vector in the process.
    // .iter_iter() 
    let score_strings: Vec<String> = scores.into_iter()
        .map(|s| format!("Score: {}", s)) // `s` is the owned i32 value
        .collect();

    println!("\n3. Consuming Iteration (collected strings):");
    println!("{:?}", score_strings);
    // println!("{:?}", scores); // Error: Value used after move, `scores` is consumed

}
