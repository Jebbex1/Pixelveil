use image::RgbImage;
use pixelveil::image_utils::open_rgbimage_from_raw;
use pyo3::{PyResult, exceptions::PyValueError};

pub(crate) fn open_image_from_bytes(image_bytes: Vec<u8>) -> PyResult<RgbImage> {
    let img = open_rgbimage_from_raw(image_bytes).map_err(|e| {
        PyValueError::new_err(format!("Failed to open image from the provided bytes. {e}"))
    })?;
    Ok(img)
}

pub(crate) fn check_rng_key_len(rng_key: &[u8]) -> PyResult<()> {
    if rng_key.len() != 32 {
        return Err(PyValueError::new_err(format!(
            "Provided RNG key length must be 32"
        )));
    }
    Ok(())
}
