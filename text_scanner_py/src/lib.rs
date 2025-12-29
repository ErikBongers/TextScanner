/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pyo3::pymodule]
pub mod text_scanner_py {
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    #[pyclass( name="WplItem")]
    #[derive(Clone)]
    pub struct WplItemPy {
        #[pyo3(get)]
        path: String,
        #[pyo3(get, name="type")]
        item_type: String
    }

    #[pyclass( name="Wpl")]
    pub struct WplPy{
        #[pyo3(get)]
        name: String,
        #[pyo3(get)]
        items: Vec<WplItemPy>
    }

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    pub fn scan_wpl(path: String) -> PyResult<WplPy> {
        let wpl = scan::scan_wpl(path)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(WplPy {
            name: wpl.name,
            items: wpl.items
                .into_iter()
                .map(|i|
                    WplItemPy{ path: i.path, item_type: i.item_type.as_str().to_string()})
                .collect()
        })
    }
}
