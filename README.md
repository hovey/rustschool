# rustschool

## Installation

* Reference: https://www.rust-lang.org/tools/install

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Use **rustup**, the Rust tool used to install, upgrade, and manage Rust.

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

## Update

Rust updates occur every six weeks.  To update Rust:

```bash
rustup update
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
