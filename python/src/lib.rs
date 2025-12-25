use pyo3::prelude::*;

mod bpcs_pyo3;
mod image_steganalysis_pyo3;
mod image_utils_pyo3;
mod pyo3_err_prop_utils;

#[pymodule(name = "pixelveil")]
fn pixelveil(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Attach bpcs submodule
    let bpcs_module = PyModule::new(py, "bpcs")?;
    bpcs_pyo3::attach_module(py, &bpcs_module)?;
    m.add_submodule(&bpcs_module)?;

    // Attach image_utils submodule
    let image_utils_module = PyModule::new(py, "image_utils")?;
    image_utils_pyo3::attach_module(py, &image_utils_module)?;
    m.add_submodule(&image_utils_module)?;

    // Attach image_steganalysis submodule
    let image_steganalysis_module = PyModule::new(py, "image_steganalysis")?;
    image_steganalysis_pyo3::attach_module(py, &image_steganalysis_module)?;
    m.add_submodule(&image_steganalysis_module)?;

    Ok(())
}
