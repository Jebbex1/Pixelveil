use pyo3::prelude::*;

mod bpcs_pyo3;
mod pyo3_err_prop_utils;

#[pymodule(name = "pixelveil")]
fn pixelveil(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Attach bpcs submodule
    let bpcs_module = PyModule::new(py, "bpcs")?;
    bpcs_pyo3::attach_module(py, &bpcs_module)?;
    m.add_submodule(&bpcs_module)?;

    Ok(())
}
