# Rust Fundamentals

## Week 1

### Module 1 (week 1)

Installing Rust and Visual Studio Code

* Install rust via [https://rustup.rs](https://rustup.rs)
* In VS Studio, install Rust Analyzer
* VS Code synchronization https://code.visualstudio.com/docs/editor/settings-sync
* Rust in VS Code: https://code.visualstudio.com/docs/languages/rust
* Codespaces - serves VS Code in the cloud
* GitHub copilot https://github.com/features/copilot
* GitHub billing and settings https://github.com/settings/billing/summary
* Developing inside a container https://code.visualstudio.com/docs/devcontainers/containers
* Microsoft's Rust First Steps https://learn.microsoft.com/en-us/training/paths/rust-first-steps/

### Module 2 (week 2)

* Reference repo: https://github.com/alfredodeza/rust-fundamentals
* `cargo init .` creates a new rust binary with the name of the folder that contains it (e.g., the '.')
* `cargo init --lib .` creates a new rust library with the same name as the containing it
* `cargo new <path>` creates a new rust package called with the name called <path>
  * `cargo new foo` creates a new rust package called `foo`
  * `cargo new --lib bar` same as above, but as a library
* `cargo build --release`
* The Cargo Book https://doc.rust-lang.org/cargo/
* Microsoft's Introduction to Rust
* Playground https://play.rust-lang.org/?version=stable&mode=debug&edition=2021
* Cooking with Rust https://rust-lang-nursery.github.io/rust-cookbook/
* Microsoft's Create your first Rust program: https://learn.microsoft.com/en-us/training/modules/rust-create-program/
* Memory management https://learn.microsoft.com/en-us/training/modules/rust-memory-management/

### Module 3 (week 3)

* Reference repo: https://github.com/alfredodeza/rust-structs-types-enums/
* Microsoft's how to loop over elements with iterators https://learn.microsoft.com/en-us/training/modules/rust-loop-expressions/
* Idiomatic to replace `if-else` statements in Rust with `enum` and `match` combinations.
* Instead of `panic!`, idiomatic to do error-handling: https://learn.microsoft.com/en-us/training/modules/rust-error-handling
  * For example, `Vec::get` returns `Option<T>`, which is `Option::Some(T)` or `Option::None`, which allows for out-of-range getting to return `None` instead of crashing.
  * Like `unwrap`, the `expect` method unwraps and adds a panic message taken as its argument.
* Instead of panicing, use pattern matching to handle the `None` case explicitly; or,
* Call a similar non-panicking method, such as `unwrap_or`, which returns the default value if the variant is `None`, and returns the inner value if the variant is `Some(value)`:

```rust
assert_eq!(Some("dog").unwrap_or("cat"), "dog");
assert_eq!(None.unwrap_or("cat"), "cat");
```

### Module 4 (week 4)

* Reference repo: https://github.com/alfredodeza/applied-rust
* Apply Rust to build a library, will debug, test, and document as if in a prefessional setting.
* `std::io::Stdin` allows Rust to take input from the user or from a pipe.

```rust
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}
```

* Microsoft's Create First Rust Program https://learn.microsoft.com/en-us/training/modules/rust-create-program/
* Debugging in VS Code https://code.visualstudio.com/docs/introvideos/debugging
* Debugging
 * https://code.visualstudio.com/docs/languages/rust
  * Install debugging support
  * To start debugging, you will first need to install one of two language extension with debugging support:
    * Microsoft C++ (ms-vscode.cpptools) – on Windows
    * CodeLLDB (vadimcn.vscode-lldb) – on macOS/Linux
* `cargo test` in addition to `cargo new`, `cargo build` and `cargo run`
* Rust standard library documentation https://doc.rust-lang.org/std/index.html
* Microsoft Explore modules, packages, and third-party crates: https://learn.microsoft.com/en-us/training/modules/rust-modules-packages-crates/

Overview of Rust programs

* A package:
  * Contains functionality within one or more crates.
  * Includes information about how to build those crates. The information is in the Cargo.toml file.
* A crate:
  * Is a compilation unit, which is the smallest amount of code that the Rust compiler can operate on.
  * Once compiled, produces either an executable or a library.
  * Contains an implicit, unnamed top-level module.
* A module:
  * Is a (possibly nested) unit of code organization inside a crate.
  * Can have recursive definitions that span additional modules.
