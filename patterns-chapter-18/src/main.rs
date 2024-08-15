fn main() {
    println!("Hello, world!");

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // let aa: Option<u8> = Some(12);
    // will not compile because the pattern is irrefutable
    let aa: Option<u8> = None;
    // let Some(x) = aa;

    if let Some(x) = aa {
        println!("{}", x);
    }

    // use of if let pattern with an irrefutable pattern produces a warning
    if let x = 5 {
        println!("x is {}", x);
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("{} is an early ASCII letter", x),
        // 'j'..='z' => println!("{} is a late ASCII letter", x),  <-- warns overlaping ranges
        'k'..='z' => println!("{} is a late ASCII letter", x),
        _ => println!("something else"),
    }

    // Destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // shorthand for destructuring structs
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10});

    let numbers = (2, 4, 6, 8, 10);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    let num = Some(4);
    
    // match guard
    // provide additional expressiveness.  The downside, the compiler doesn't
    // try to check for exhaustiveness when match guard exprssions are used.
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

}
