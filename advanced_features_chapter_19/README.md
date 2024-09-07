# Advanced Features

These are not common, so this chapter is an overview as a final look at Rust
features, prior to delving into the final project in Chapter 20.

This Advanced Features chapter should be used as reference.

* Unsafe Rust
* Advanced traits - associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern and its relation to traits
* Advanced types
* Advanced functions and closures
* Macros

## Unsafe Rust

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

// Stop 2024-09-09, finished Section 19.1.  Next, start Section 19.2.
