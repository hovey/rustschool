# rustschool

## Installation

* Reference: https://www.rust-lang.org/tools/install

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Use **rustup**, the Rust tool used to install, upgrade, and manage Rust.

```bash
 chovey@s1088757/Users/chovey> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /Users/chovey/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /Users/chovey/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /Users/chovey/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /Users/chovey/.profile
  /Users/chovey/.bash_profile
  /Users/chovey/.zshenv
  /Users/chovey/.config/fish/conf.d/rustup.fish

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: aarch64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>1

info: profile set to 'default'
info: default host triple is aarch64-apple-darwin
warning: Updating existing toolchain, profile choice will be ignored
info: syncing channel updates for 'stable-aarch64-apple-darwin'
info: latest update on 2024-05-02, rust version 1.78.0 (9b00956e5 2024-04-29)
info: downloading component 'rust-src'
info: downloading component 'cargo'
  6.3 MiB /   6.3 MiB (100 %)   2.9 MiB/s in  1s ETA:  0s
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 15.1 MiB /  15.1 MiB (100 %)   7.3 MiB/s in  2s ETA:  0s
info: downloading component 'rust-std'
 22.6 MiB /  22.6 MiB (100 %)   7.3 MiB/s in  3s ETA:  0s
info: downloading component 'rustc'
 50.7 MiB /  50.7 MiB (100 %)   8.6 MiB/s in  6s ETA:  0s
info: downloading component 'rustfmt'
info: removing previous version of component 'rust-src'
info: removing previous version of component 'cargo'
info: removing previous version of component 'clippy'
info: removing previous version of component 'rust-docs'
info: removing previous version of component 'rust-std'
info: removing previous version of component 'rustc'
info: removing previous version of component 'rustfmt'
info: installing component 'rust-src'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 15.1 MiB /  15.1 MiB (100 %)   2.1 MiB/s in  5s ETA:  0s
info: installing component 'rust-std'
 22.6 MiB /  22.6 MiB (100 %)  18.4 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 50.7 MiB /  50.7 MiB (100 %)  19.6 MiB/s in  2s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-aarch64-apple-darwin'

  stable-aarch64-apple-darwin updated - rustc 1.78.0 (9b00956e5 2024-04-29) (from rustc 1.74.0 (79e9716c9 2023-11-13))


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
 chovey@s1088757/Users/chovey> source "$HOME/.cargo/env.fish:"
source: Error encountered while sourcing file '/Users/chovey/.cargo/env.fish:':
source: No such file or directory
 chovey@s1088757/Users/chovey> source "$HOME/.cargo/env.fish"
 chovey@s1088757/Users/chovey> which cargo
/Users/chovey/.cargo/bin/cargo
 chovey@s1088757/Users/chovey> which rustup
/Users/chovey/.cargo/bin/rustup
```

## Update

Rust updates occur every six weeks.  To update Rust:

```bash
rustup update
```

## Run

To run

```bash
$ rustc main.rs
$ ./main
Hello, world!
```

## Tutorials

```bash
git clone https://github.com/kyclark/command-line-rust.git
```

## Create and Run with Cargo

```bash
cargo new <project>

# example
cargo new hello

# run
cargo run
cargo run --quiet
cargo run -q

# just build
cargo build  # build, but not run
```

## Test

```bash
cargo test
```

## VS Code Extension

* [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

```bash
https://rust-analyzer.github.io/manual.html
```

## "The Rust Programming Language" book

To get started, open the local "The Rust Programming Language" with

```bash
chovey@s1088757/Users/chovey/command-line-rust> rustup docs --book
```

which will open the local documentation in a web browser.

## Anirudh 2024-01-26

* https://doc.rust-lang.org/nomicon/
* For Michael, Traits in Julia, not built-in as with Rust, but Julia libraries can provide this functionality.
  * https://juliapackages.com/p/traits
  * Rust traits are nicer than Julia b/c Rust are built in, and first-class citizens
* https://www.zsa.io/moonlander/buy
* https://www.sennheiser.com/en-us/catalog/products/microphones/profile-usb-microphone/profile-usb-microphone-700065

### Bookmarks

* 2024-01-25: Finished through 3.4 Comments.
* 2024-01-23: Finished Chapter 2, start next on Chapter 3.
* 2024-01-12: file:///Users/chovey/.rustup/toolchains/stable-aarch64-apple-darwin/share/doc/rust/html/book/ch02-00-guessing-game-tutorial.html#testing-the-first-part


## Deprecated

The previous installation, with bash (not fish):

```bash
bash-3.2$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /Users/chovey/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /Users/chovey/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
The bin directory for Cargo, located at:

  /Users/chovey/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /Users/chovey/.profile
  /Users/chovey/.bash_profile
  /Users/chovey/.zshenv

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: aarch64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation

>1

info: profile set to 'default'
info: default host triple is aarch64-apple-darwin
info: syncing channel updates for 'stable-aarch64-apple-darwin'
info: latest update on 2023-11-16, rust version 1.74.0 (79e9716c9 2023-11-13)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 14.4 MiB /  14.4 MiB (100 %)  12.4 MiB/s in  1s ETA:  0s
info: downloading component 'rust-std'
 24.1 MiB /  24.1 MiB (100 %)   7.8 MiB/s in  3s ETA:  0s
info: downloading component 'rustc'
 54.6 MiB /  54.6 MiB (100 %)   3.1 MiB/s in 15s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 14.4 MiB /  14.4 MiB (100 %)   2.1 MiB/s in  4s ETA:  0s
info: installing component 'rust-std'
 24.1 MiB /  24.1 MiB (100 %)  19.2 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 54.6 MiB /  54.6 MiB (100 %)  21.2 MiB/s in  2s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-aarch64-apple-darwin'

  stable-aarch64-apple-darwin installed - rustc 1.74.0 (79e9716c9 2023-11-13)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
The bin directory for Cargo ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
bash-3.2$
```
