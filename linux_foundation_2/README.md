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



