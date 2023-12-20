use pyo3::prelude::*;

use crate::client::InfisicalClient;

#[pymodule]
fn infisical_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<InfisicalClient>()?;
    Ok(())
}
