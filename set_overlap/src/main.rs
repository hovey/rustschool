
// HashSet collection has tools for determining set relationships, such as
// subset.
use std::collections::HashSet;

fn main() {
    /*
    // Define two sets
    // The iter().cloned().collect() converts an array into a HashSet
    let set_aa: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set_bb: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();

    // Is set_aa a subset of set_bb?  Yes.
    if set_aa.is_subset(&set_bb) {
        println!("set_aa is a subset of set_bb");
    } else {
        println!("set_aa is not a subset of set_bb");
    }

    // Is set_bb a subset of set_aa?  No.
    if set_bb.is_subset(&set_aa) {
        println!("set_bb is a subset of set_aa");
    } else {
        println!("set_bb is not a subset of set_aa");
    }
    */

    // input vectors typically arrive into function a two vectors of i8 values
    // use Integer Type length 8-bit, unsigned, u8
    let vec_a: Vec<u8> = vec![0, 0, 2, 2, 4, 4, 6, 6];
    let vec_b: Vec<u8> = vec![1, 2, 3, 4];
    let vec_c: Vec<u8> = vec![5, 6, 7, 8];
    println!("vec_a {:?}", vec_a);
    println!("vec_b {:?}", vec_b);
    println!("vec_c {:?}", vec_c);

    // let set_a = HashSet::from(vec_a);
    // let set_a = HashSet::from([0, 2, 4, 6]);
    // let set_b = HashSet::from([1, 2, 3, 4]);
    // let set_c = HashSet::from([5, 6, 7, 8]);
    // println!("set_a {:?}", set_a);
    // println!("set_b {:?}", set_b);
    // println!("set_c {:?}", set_c);

    // convert the vector to a HashSet
    // A HashSet eliminates duplicates and only allows unique elements
    let set_a: HashSet<u8> = vec_a.into_iter().collect();
    let set_b: HashSet<u8> = vec_b.into_iter().collect();
    let set_c: HashSet<u8> = vec_c.into_iter().collect();

    // let a_and_b: HashSet<_> = set_a.intersection(&set_b).collect();
    // let b_and_c: HashSet<_> = set_b.intersection(&set_c).collect();
    // let c_and_a: HashSet<_> = set_c.intersection(&set_a).collect();
    let a_and_b: HashSet<u8> = set_a.intersection(&set_b).cloned().collect();
    let b_and_c: HashSet<u8> = set_b.intersection(&set_c).cloned().collect();
    let c_and_a: HashSet<u8> = set_c.intersection(&set_a).cloned().collect();
    println!("intersection a_and_b {:?} with length: {}", a_and_b, a_and_b.len());
    println!("intersection b_and_c {:?} with length: {}", b_and_c, b_and_c.len());
    println!("intersection c_and_a {:?} with length: {}", c_and_a, c_and_a.len());

    let mut vec_of_sets: Vec<HashSet<u8>> = Vec::new();
    vec_of_sets.push(a_and_b);
    vec_of_sets.push(b_and_c);
    vec_of_sets.push(c_and_a);

    for set in vec_of_sets {
        if set.len() == 0 {
            println!("set {:?} is an empty set", set)
        } else {
            println!("set {:?} is not an empty set", set)
        }
    }
}
