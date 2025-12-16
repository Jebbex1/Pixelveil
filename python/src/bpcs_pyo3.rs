use pixelveil::image_utils::{export_image_to_png_bytes, open_rgbimage_from_raw};
use pyo3::prelude::*;

#[pyfunction]
fn embed_data(
    source_image_bytes: Vec<u8>,
    data: Vec<u8>,
    min_alpha: f64,
    rng_key: Vec<u8>,
) -> PyResult<Vec<u8>> {
    assert_eq!(rng_key.len(), 32);

    let mut source_image = open_rgbimage_from_raw(source_image_bytes).unwrap();
    let data_length = data.len();

    pixelveil::bpcs::embed_data(
        &mut source_image,
        &mut data.into_iter(),
        data_length,
        min_alpha,
        rng_key.try_into().unwrap(),
    )
    .unwrap();

    Ok(export_image_to_png_bytes(&source_image))
}

#[pymodule(name = "bpcs")]
pub(crate) fn attach_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(embed_data, m)?)?;

    Ok(())
}
