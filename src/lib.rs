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