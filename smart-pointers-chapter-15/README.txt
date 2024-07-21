Chapter 15: Smart Pointers

A pointer is a variable that "points to" a location in memory.

The most common Rust pointer is a reference, indicated by the & symbol, and
which borrows the data it points to.

Relative to pointers, smart pointers have additional metadata and capabilities:

Smart pointers have reference counting, which allows data to have multiple
owners, and when no more owners remain, cleaning up the data.

There is a difference between owning and borrowing.
References borrow the data, whereas smart pointers can *own* the data they
point to.

Smart pointers are ususaly implemented using Structs.  Unlike an ordinary
struct, smart pointers implement the Deref and Drop traits.
The Deref trait allows smart pointers to be used in the same way as regular
pointers.
The Drop trait allows the developer to customize code that is run once a smart
pointer goes out of scope.

The most common smart pointers in the standard library:
Box<T> for allocating values on the heap
Rc<T>, a reference counting type that enables multiple ownership
Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the
borrowing rules at runtime instead of at compile time.

Interior mutability pattern, where an immutable type exposes and API for
mutating an interior value.

Reference cycles: how they can leak memory and how to prevent leaks.

Section 15.1: Using Box<T> to Point to Data on the heap

The most straightforward smart pointer is a box, written Box<T>, allows one to
store data on the heap instead of the stack.  What remains on the stack is the
pointer to the data on the heap.  Used for data

* when the size of that data is unknown at compile time
* when there is a large amount of data, and one wants to transfer ownership, but
ensure the data won't be copied when ownership is transferred
* when one wants to own a value, and one only cares that the value is a type that
implements a particular trait, rather than being a specific type (so, heterogeneous
pointers).   This is called a trait object, discussed in Chapter 17, Section 17.2,
"Using trait objects that allow for values of different types."

Box<T> is a form of indirection, that is, storing the data value indirectly by
storing a value to a pointer instead of the data itself.

Implementing the Defer trait allows one to use the dereference operator *
(not the multiplication operator *, and not the glob operator *).  Implement
the Defer trait to allow smart pointers to be used like a regular pointers.
Code then can work on smart pointers in exactly the same way that it operates
on regular pointers.
