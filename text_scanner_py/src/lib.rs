/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pyo3::pymodule]
pub mod text_scanner_py {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    pub fn sum_as_str(a: usize, b: usize) -> PyResult<String> {
        Ok(text_scanner::sum_as_string(a, b))
    }
}

