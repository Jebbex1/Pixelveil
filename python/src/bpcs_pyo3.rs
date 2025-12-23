use crate::pyo3_err_prop_utils::{check_rng_key_len, open_image_from_bytes};
use pixelveil::{errors::SteganographyError, image_utils::export_image_to_png_bytes};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction]
fn embed_data(
    vessel_image_bytes: Vec<u8>,
    data: Vec<u8>,
    min_alpha: f64,
    rng_key: &[u8],
) -> PyResult<Vec<u8>> {
    check_rng_key_len(rng_key)?;

    let mut vessel_image = open_image_from_bytes(vessel_image_bytes)?;

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
        SteganographyError::InsufficientPlaneNumber(expected, got) => PyValueError::new_err(
            format!("Tried to embed {expected} planes when only {got} planes were available."),
        ),
        _ => {
            panic!("This error is not supposed to be propagated when embedding with BPCS. Please contact the developer");
        },
    })?;

    Ok(export_image_to_png_bytes(&vessel_image))
}

#[pyfunction]
fn extract_data(vessel_image_bytes: Vec<u8>, min_alpha: f64, rng_key: &[u8]) -> PyResult<Vec<u8>> {
    check_rng_key_len(rng_key)?;

    let vessel_image = open_image_from_bytes(vessel_image_bytes)?;

    let extracted = pixelveil::bpcs::extract_data(
        vessel_image,
        min_alpha,
        rng_key.try_into().unwrap(),
    )
    .map_err(|e| match e {
        // only InsufficientPlaneNumber can be raised while embedding with BPCS
        SteganographyError::InsufficientPlaneNumber(expected, got) => PyValueError::new_err(
            format!(
                "Tried to read {expected} planes from the image when there were only {got} valid ones. 
                This was probably caused by attempting to extract data from an image that didn't have data hidden in it 
                or with incorrect parameters. If you know this is not the case, please contact the developer"
            ),
        ),
        SteganographyError::InvalidIVData(reason) => PyValueError::new_err(
            format!(
                "The IV that was read contains invalid data: {reason}
                This was probably caused by attempting to extract data from an image that didn't have data hidden in it 
                or with incorrect parameters. If you know this is not the case, please contact the developer"
            ),
        ),
    })?;

    Ok(extracted)
}

#[pyfunction]
fn estimate_maximum_capacity(vessel_image_bytes: Vec<u8>, min_alpha: f64) -> PyResult<u64> {
    let vessel_image = open_image_from_bytes(vessel_image_bytes)?;

    Ok(pixelveil::bpcs::estimate_maximum_capacity(
        &vessel_image,
        min_alpha,
    ))
}

#[pymodule(name = "bpcs")]
pub(crate) fn attach_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(embed_data, m)?)?;
    m.add_function(wrap_pyfunction!(extract_data, m)?)?;
    m.add_function(wrap_pyfunction!(estimate_maximum_capacity, m)?)?;

    Ok(())
}
