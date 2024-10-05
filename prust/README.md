# Prust

Rust, wrapped with Python.

## Configuration

Create a new `rust` project

```sh
cd ~/rustschool/prust/
cargo new --lib showcase
```

Update the [`~/rustschool/prust/showcase/cargo.toml`](showcase/Cargo.toml) as follows,

```toml
[package]
name = "showcase"
version = "0.1.0"
edition = "2021"

[dependencies]

[lib]
name = "showcase"
# "cdylib" is necessary to produce a shared library for Python to import from.
crate-type = ["cdylib"]

[dependencies.pyo3]
version = "0.21.1"
# "abi3-py38" tells pyo3 (and maturin) to build using the stable ABI with minimum Python version 3.8
features = ["abi3-py38"]
```

`maturin` is configured in
[`~/rustschool/prust/showcase/pyproject.toml`](showcase/pyproject.toml)
as introduced by
[PEP 518](https://peps.python.org/pep-0518/).
This file lives in the root of the project tree:

```sh
showcase/
├── Cargo.toml
├── pyproject.toml  #  <<< add this file
└── src
    └── lib.rs
```

In the `pyproject.toml`, set the 
build system requirements and enable the `extension-module` feature of `pyo3`.

```sh
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[tool.maturin]
# "extension-module" tells pyo3 we want to build an extension module (skips linking against libpython.so)
features = ["pyo3/extension-module"]
```

Install and configure `maturin`:


```sh
cd ~/rustschool/prust/showcase/
python3 -m venv .venv
source .venv/bin/activate.fish
pip install -U pip maturin
```

Note that `maturin` has minimual dependencies:

```sh
pip freeze
maturin==1.7.4
```

### Rust Service

When creating a `lib` project with `cargo new`, `cargo` creates a file
called `src/lib.rs` with some default code.

Edit the [`~/rustschool/prust/showcase/src/lib.rs`](showcase/src/lib.rs)
file to appear as follows:

```rust
use pyo3::prelude::*;

#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn showcase(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    Ok(())
}
```

### Build and Install the Python module with `maturin develop`

This is just a Rust project at this point.
So, it can be built using `cargo build`.
However, `maturin` can be used instead, as it adds some platform-specific
build configurations and packages the binary result as a wheel.
A wheel file as the `.whl` file extension.  A wheel file is an archive of
compiled components that can be installed using `pip`, the Python package
manager.

So let's use maturin to build and install in our current environment.

```sh
    Updating crates.io index
🔗 Found pyo3 bindings with abi3 support for Python ≥ 3.8
🐍 Not using a specific python interpreter
📡 Using build options features from pyproject.toml
   Compiling target-lexicon v0.12.16
   Compiling once_cell v1.20.1
   Compiling autocfg v1.4.0
   Compiling proc-macro2 v1.0.86
   Compiling libc v0.2.159
   Compiling unicode-ident v1.0.13
   Compiling parking_lot_core v0.9.10
   Compiling cfg-if v1.0.0
   Compiling portable-atomic v1.9.0
   Compiling heck v0.4.1
   Compiling smallvec v1.13.2
   Compiling scopeguard v1.2.0
   Compiling indoc v2.0.5
   Compiling unindent v0.2.3
   Compiling lock_api v0.4.12
   Compiling memoffset v0.9.1
   Compiling quote v1.0.37
   Compiling syn v2.0.79
   Compiling pyo3-build-config v0.21.2
   Compiling parking_lot v0.12.3
   Compiling pyo3-ffi v0.21.2
   Compiling pyo3 v0.21.2
   Compiling pyo3-macros-backend v0.21.2
   Compiling pyo3-macros v0.21.2
   Compiling showcase v0.1.0 (/Users/chovey/rustschool/prust/showcase)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
📦 Built wheel for abi3 Python ≥ 3.8 to /var/folders/87/_m6404ms1hd30kz498l9bt8w002wpg/T/.tmpaHHYFa/showcase-0.1.0-cp38-abi3-macosx_11_0_arm64.whl
✏️  Setting installed package as editable
🛠 Installed showcase-0.1.0
```


One can also compile a performance-optimized program by adding 
the `-r` or `--release` flag for speed testing.

### Python REPL

```sh
# from ~/rustschool/prust/showcase> python
python
>>> import showcase as sc
>>> sc.multiply(2, 3)
6
```

It works!

Note that the module has been installed locally:

```sh
(.venv)  (main) chovey@mac/Users/chovey/rustschool/prust/showcase> pip list
Package    Version Editable project location
---------- ------- ---------------------------------------
maturin    1.7.4
pip        24.2
setuptools 65.5.0
showcase   0.1.0   /Users/chovey/rustschool/prust/showcase
```

### Python Client

Create [`~/rustschool/prust/showcase/multiply.py`](showcase/multiply.py)
with the following contents,

```python
import showcase as sc

result = sc.multiply(2, 3)
print(result)
```

## References

S van de Klundert.  Calling Rust from Python using PyO3, Speed up your Python
using Rust, 18 Nov 2021 [link](https://saidvandeklundert.net/learn/2021-11-18-calling-rust-from-python-using-pyo3/) *This example has some
deprecated rust constructs, unfortunately.*

PyO3 and Maturin: https://github.com/PyO3/maturin

Maturin User Guide: https://www.maturin.rs

Maturin Tutorial: https://www.maturin.rs/tutorial
