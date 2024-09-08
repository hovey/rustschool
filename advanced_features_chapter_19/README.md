# Chapter 19: Advanced Features

These are not common, so this chapter is an overview as a final look at Rust
features, prior to delving into the final project in Chapter 20.

This Advanced Features chapter should be used as reference.

* Unsafe Rust
* Advanced traits - associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern and its relation to traits
* Advanced types
* Advanced functions and closures
* Macros

## Section 19.1: Unsafe Rust

Rust has a memory safety guarantee, enforced by the compiler.
However, *unsafe Rust* is a second lanugage hidden inside that doesn't enforce
the memory safety guarantees.

Unsafe Rust is useful for some cases. E.g., working with underlying computer
hardware.

`unsafe` doesn't turn off the borrow checker, nor does it disable any other of
Rust's safety checks.  

Best practice - enclose unsafe code with a safe abstraction and provide a
safe API.

One of the biggest reasons to use raw pointers is to interface with C code.

### Dereferencing a Raw Pointer

As with references, raw pointers, can be immutable or mutable, written
as `*const T` and `*mut T`, respectively.  This is *not* the deference
operator.

Unlike references and smart pointers, raw pointers

* are allowed to ignore the borrowing rules
* aren't guaranteed to point to valid memory
* are allowed to be null
* don't implement any automatic cleanup

```rust
// create an immutable and a mutable raw pointer from references
// by casting using the `as` keyword
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

// To use the dereference operator `*` on a raw pointer requires the
// `unsafe` block
unsafe {
    println!("r1 is {}," *r1);
    println!("r2 is {}", *r2);
}
```

### Calling an unsafe function or method

```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

### Create a Safe Abstraction over Unsafe Code

Wrapping unsafe code in a safe function is a common abstraction.

From "The Slice Type" in Chapter 4, slices are a pointer to some data, and the
length of the slice.

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mu [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

### Using `extern` Functions to Call External Code

```rust
// The "C" part defines the Application Binary Interface (ABI), which defines
// how to call the function at the assembly level.  The "C" ABI is the most
// common and follows the C programming language's ABI.
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

We can also create an interface that allows other languages to call Rust
functions.  We must tell the Rust compi8ler not to mangle the name of the
function.

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

### Accessing or Modifying a Mutable State Variable

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is {}", HELLO_WORLD);
}
```

A subtle difference between constants and immutable static variables is that
values in a static variable have a fixed address in memory; use of the value
will always access the same data.  Constants, however, are allowed to duplicate
their data whenevery they are used.  

Another difference: static variables can be mutable.  Accessing and modifying
mutable static variabless is *unsafe*.

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

The code above will likely result in a data race if mutiple threads are used.

### Implementing an Unsafe Trait

Both the trait and the implementation get the `unsafe` keyword.

```rust
unsafe trait Foo {
    // methods go here
}
unsafe impl Foo for i32 {
    // method implementations go here
}

fn main {}
```

### Accessing Fields of a Union

A `union` is similar to a `struct`, but only declared field is used in a
particular instance at one time.  Unions are primarily used to interface
with unions in C code.

// Stop 2024-09-07, finished Section 19.1.  Next, start Section 19.2.

## Section 19.2: Advanced Traits

### Associated Types

Associated types connect a *type* placeholder with a trait so that trait
method definitions can use these placeholder types in their signatures.

Associated types are similar to generics.  

```rust
// associated type
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
}

// generic
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

Hmmmm... clear as mud right now.

### Default Generic Type Paramters and Operator Overloading

Example of operator overloading: customize the behavior of an operator,
such as `+` operator.  One can overload the operations and corresponding traits
listed in `std::ops`` by implementing the traits associated with the operator.

For example, we could overload the `+` operator to add to `Point` instances
together.  We implement the `Add` trait on a `Point struct:

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

The default generic type in this code within the `Add` trait is:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

The `Rhs=Self` syntax is called a *default type parameter*.

Example of adding where we want to customize the `Rhs` type:

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

### Fully Qualitifed Syntax for Disambiguation

*Calling Methods with the Same Name*

Example: define two traits, `Pilot` and `Wizard`, which both have a method
called `fly`.  When implementing both traits on a type `Human`, we must
get specific:

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    // first trait implementation
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    // second trait implementation
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    // direct implementation
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

When we call `fly` on the instance of `Human`, the compiler defaults to the
method that is directly implemented as shown below:

```rust
fn main() {
    let person = Human;
    person.fly();  // prints *waving arms furiously*
    // Equivalent to person.fly() is Human::fly(&person), the latter is the
    // disambiguation signature

    // To call the other `fly` methods, be more explicit:
    Pilot::fly(&person);
    Wizard::fly(&person);
}
```

*Fully qualified syntax*

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    // Will print, `A baby dog is called a Spot

    // following will cause a compiler error:
    println!("A baby dog is called a {}", Animal::baby_name());
    // because rust can't figure out which implementation of Animal::baby_name
    // we want, so we must disambiguate:

    // to disambiguate
    println!("A baby dog is called a {}," <Dog as Animal>::baby_name());

    // a fully qualified syntax example:
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
}
```

### Use of Supertraits

*Supertraits can be used to require one trait's functionality within another trait*

Sometimes, one might one trait to depend on another trait, for a type to
implement the first trait, we want to require that the the type also implements
the second trait.  

Example: we want the `outline_print` to use the `Display` trait's
functionality.  We need to specifiy that the `OutlinePrint` trait will
only work for types that also implement the `Display` trait.  We accomplish
this in the trait definition by specifying `OutlinePrint: Display`.
This technique is simplar to adding a trait bound to the trait.

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", "*".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

Then for the `Point`:


```rust
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {} // compile error, `Point cannot be formatted with the default formatter
```

The fix: implement `Display` on `Point` and satisfy the constraint that
`OutlinePrint` requires:

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x,)
    }
}
```

then we can call `outline_print` on a `Point` instance to display it
within an outline of asterisks, as desired.

### Use of the Newtype Pattern

*Use the Newtype pattern to implement external traits on external types*

The *Newtype* term originates from the Haskell programming language.
There is no performance penality for using this pattern, and the wrapper
type is elided a compile time.

Example: we want to implment `Display` on `Vec<T>`, but the ophan rule prevents
us from doing directly because the `Display` trait and the `Vec<T>` type
are defined outside of our crate.  We can make a `Wrapper` struct that holds
an instance of `Vec<T>`, athen implement `Display` on `Wrapper` and use
the `Vec<T>` value, as shown below:

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {} {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

Hmmmm... will have to make this more concrete as I continue...

2024-09-08: finish Section 19.2.  Next, start on Section 19.3.
