Day 1

Instructor: Pascal
pascal@poortier.nl

Dave used Helix, editor written in Rust
https://helix-editor.com

Types on the stack (simple types) automatically get copied.
Types on the heap (complex types) have pointers to data that is not copied, and better for demonstrating ownership.

Copy trait and clone type make this distiction.

For growable data types, not implement the copy trait, rather 
(a) implement the clone type, or
(b) use heap memory locations

Copy for stack types.
Clone for heap types.

Clone often necessary with closures.
Clone often needed with concurrency.

Clone is a super-type of Copy.

Copy is very cheap stack operation.
Clone can potentially be expensive, and may involve heap allocation.

Const versus Static

* const is preferred to statc because const creates inline literal substitutions, and is therefore fast.
Whereas, static is a read-only memory reference, so it can be slow.
Use of static for values that must have a fixed memory location and are valid for the entire duration of the program.
Prefer const to static.
Use case for using static is for interfacing with C-code or other foreign-function interfaces (FFI).

Day 2

Edge cases

NLL - non-lexical lifetimes

Partial borrowing
ex 1:
    let mut nums = [1, 2, 3, 4, 5, 6];
    let (head, tail) = nums.split_at_mut(3);


match guards and alias

Binding


Day 3

Custom error handling
https://crates.io/crates/thiserror (604m downloads)
https://crates.io/crates/anyhow (450m downloads)
ptr_info_lib (by Pascal Van Dam)
https://crates.io/crates/ptr_info_lib
Poortier Management B.V.
Bergen (Limburg) NL
https://github.com/pascal71


casting
* use from() and into(), and returns if the cast was or was not successful
use std::convert::TryInto

.tryinto() versus .into()

Projects hierarchy
- Package is a bundle of binary (or binaries) and (not more than one) library crate.
- - Crate a is a compilation unit, either a binary crate or a library crate.
- - - A module is namespace.
- - - - Paths - absolute or relative paths


Rust does not have coverage, but crate: cargo-tarpaulin (2m downloads)

Iterators

lazy evaluation, only does the evalution until user asks for next item (thus efficient)

* adapter iterators, iterator goes in and another iterator comes out
* consumer iterators, iterator goes in and a type comes out

.filter.map transform into a new item and collect the result into a new collection (a consuming operation)
.filter.for_each to execute in-place mutation

Associated function, such as new (Constructor), called with Type::new(), does not take self.
Versus a method, takes self, and uses the instance.method(...) pattern

Chad prefers to use the ::new() asssociated function because it can check state is valid
on construction, whereas without the ::new() there is no way to guard against directly creating
bad Structs/Enums e.g., radius with negative values, bad path variables.


Day 4

cargo-nextest with .config/nextest.toml for testing with CI/CD
https://nexte.st

Trait bounds, useful but avoid overuse because it can be overly restrictive.
Really we want to create abstraction through Traits, so to bound the trait with trait
bounds dilutes the effectiveness of the trait.

Prefer small, focused traits over large traits.  Large traits make the traits weakers
and less used.

Newtype pattern, it is a tuple struct.

Marker traits - traits without methods.

Page 817, dynamics dispatch, cat and dog speaking example.

Need to show an example of static versus dynamic dispatch.

Generics

