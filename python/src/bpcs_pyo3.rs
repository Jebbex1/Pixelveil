use pixelveil::{
    errors::SteganographyError,
    image_utils::{export_image_to_png_bytes, open_rgbimage_from_raw},
};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction]
fn embed_data(
    vessel_image_bytes: Vec<u8>,
    data: Vec<u8>,
    min_alpha: f64,
    rng_key: &[u8],
) -> PyResult<Vec<u8>> {
    if rng_key.len() != 32 {
        return Err(PyValueError::new_err(format!(
            "Provided RNG key length must be 32"
        )));
    }

    let mut vessel_image = open_rgbimage_from_raw(vessel_image_bytes).map_err(|e| {
        PyValueError::new_err(format!("Failed to open image from the provided bytes. {e}"))
    })?;

    let data_length = data.len();

    pixelveil::bpcs::embed_data(
        &mut vessel_image,
        &mut data.into_iter(),
        data_length,
        min_alpha,
        rng_key.try_into().unwrap(),
    )
    .map_err(|e| match e {
        // only InsufficientPlaneNumber can be raised while embedding with BPCS
        SteganographyError::InsufficientPlaneNumber(expected, got) => {
            PyValueError::new_err(format!("Tried to embed {expected} planes when only {got} planes were available."))
        }
        _ => PyValueError::new_err(format!("{e}")),
    })?;

    Ok(export_image_to_png_bytes(&vessel_image))
}

#[pymodule(name = "bpcs")]
pub(crate) fn attach_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(embed_data, m)?)?;

    Ok(())
}
