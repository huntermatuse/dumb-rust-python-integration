# a dumb rust and python integration

this project is just me testing how to connection a rust function into something that i can call from python using PyO3. 

the following was written by llms after i wrote the project locally and asked it nicely to write up a readme.

rust `i64` type is used, which could potentially overflow for very large integers. for this example that problem remains out of scope. 

## Project Structure

```
rustsum/
├── Cargo.toml
├── src/
│   └── lib.rs
├── README.md
python/
└── testing.py
```


### 1. Rust build

(`src/lib.rs`):

```rust
use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;

#[pyfunction]
fn sum(a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

#[pymodule]
fn rustsum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum, m)?)?;
    Ok(())
}
```

- The `#[pyfunction]` attribute makes the `sum` function callable from Python.
- The `#[pymodule]` attribute defines the module that will be imported in Python.

### 2. Cargo Configuration

The `Cargo.toml` file configures the Rust project:

```toml
[package]
name = "rustsum"
version = "0.1.0"
edition = "2021"

[lib]
name = "rustsum"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.18.0", features = ["extension-module"] }
```

- `crate-type = ["cdylib"]` specifies that we're building a dynamic library.
- The `pyo3` dependency allows Rust to interact with Python.

### 3. Building the Extension

We use `maturin` to build the Rust extension:

1. Install maturin: `pip install maturin`
2. Build the extension: `maturin develop`

### 4. Using in Python

After building, you can use the Rust function in Python:

```python
from rustsum import sum

result = sum(5, 7)
print(f"The sum is: {result}")
```

## Benefits and Considerations

1. **Performance**: Rust can provide performance improvements for computationally intensive tasks.
2. **Type Safety**: Rust's strong type system can catch errors at compile-time.
3. **Interoperability**: You can use Rust to extend existing Python projects or libraries.

However, note that this simple example doesn't showcase significant performance benefits. Complex calculations or data processing tasks would demonstrate Rust's advantages more clearly.

## Next Steps

- Implement more complex functions to leverage Rust's performance.
- Add error handling and input validation.
- Explore using Rust's standard library or third-party crates to extend functionality.
