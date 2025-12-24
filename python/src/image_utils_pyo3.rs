use image::Rgb;
use pixelveil::image_utils::export_image_to_png_bytes;
use pyo3::prelude::*;

use crate::pyo3_err_prop_utils::open_image_from_bytes;

#[pyfunction]
fn pixel_to_gray_code(pixel: (u8, u8, u8)) -> PyResult<(u8, u8, u8)> {
    let mut rgb_pixel = Rgb::<u8> {
        0: [pixel.0, pixel.1, pixel.2],
    };

    pixelveil::image_utils::pixel_to_gray_code(&mut rgb_pixel);

    Ok((rgb_pixel.0[0], rgb_pixel.0[1], rgb_pixel.0[2]))
}

#[pyfunction]
fn pixel_to_binary_code(pixel: (u8, u8, u8)) -> PyResult<(u8, u8, u8)> {
    let mut rgb_pixel = Rgb::<u8> {
        0: [pixel.0, pixel.1, pixel.2],
    };

    pixelveil::image_utils::pixel_to_binary_code(&mut rgb_pixel);

    Ok((rgb_pixel.0[0], rgb_pixel.0[1], rgb_pixel.0[2]))
}

#[pyfunction]
fn image_to_gray_code(image_bytes: Vec<u8>) -> PyResult<Vec<u8>> {
    let mut img = open_image_from_bytes(image_bytes)?;

    pixelveil::image_utils::image_to_gray_code(&mut img);

    Ok(export_image_to_png_bytes(&img))
}

#[pyfunction]
fn image_to_binary_code(image_bytes: Vec<u8>) -> PyResult<Vec<u8>> {
    let mut img = open_image_from_bytes(image_bytes)?;

    pixelveil::image_utils::image_to_binary_code(&mut img);

    Ok(export_image_to_png_bytes(&img))
}

#[pymodule(name = "image_utils")]
pub(crate) fn attach_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pixel_to_gray_code, m)?)?;
    m.add_function(wrap_pyfunction!(pixel_to_binary_code, m)?)?;
    m.add_function(wrap_pyfunction!(image_to_gray_code, m)?)?;
    m.add_function(wrap_pyfunction!(image_to_binary_code, m)?)?;

    Ok(())
}
