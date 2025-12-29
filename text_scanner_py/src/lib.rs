/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pyo3::pymodule]
pub mod text_scanner_py {
    use pyo3::prelude::*;
    use pyo3::exceptions::PyValueError;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    pub fn scan_wpl(path: String) -> PyResult<Vec<String>> {
        let tracks = scan::scan_wpl(path)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(tracks)
    }
}
