use std::collections::HashMap;

use image::Rgb;
use pixelveil::image_utils::export_image_to_png_bytes;
use pyo3::prelude::*;

use crate::pyo3_err_prop_utils::open_image_from_bytes;

#[pyfunction]
fn subtract_pixels(pixel1: (u8, u8, u8), pixel2: (u8, u8, u8)) -> PyResult<(u8, u8, u8)> {
    let rgb_pixel1 = Rgb::<u8> {
        0: [pixel1.0, pixel1.1, pixel1.2],
    };

    let rgb_pixel2 = Rgb::<u8> {
        0: [pixel2.0, pixel2.1, pixel2.2],
    };

    let subtracted = pixelveil::image_steganalysis::subtract_pixels(&rgb_pixel1, &rgb_pixel2);

    Ok((subtracted.0[0], subtracted.0[1], subtracted.0[2]))
}

#[pyfunction]
fn xor_pixels(pixel1: (u8, u8, u8), pixel2: (u8, u8, u8)) -> PyResult<(u8, u8, u8)> {
    let rgb_pixel1 = Rgb::<u8> {
        0: [pixel1.0, pixel1.1, pixel1.2],
    };

    let rgb_pixel2 = Rgb::<u8> {
        0: [pixel2.0, pixel2.1, pixel2.2],
    };

    let xored = pixelveil::image_steganalysis::xor_pixels(&rgb_pixel1, &rgb_pixel2);

    Ok((xored.0[0], xored.0[1], xored.0[2]))
}

#[pyfunction]
fn subtract_images(image1_bytes: Vec<u8>, image2_bytes: Vec<u8>) -> PyResult<Vec<u8>> {
    let image1 = open_image_from_bytes(image1_bytes)?;
    let image2 = open_image_from_bytes(image2_bytes)?;

    let subtracted = pixelveil::image_steganalysis::subtract_images(&image1, &image2);

    Ok(export_image_to_png_bytes(&image::DynamicImage::ImageRgb8(
        subtracted,
    )))
}

#[pyfunction]
fn xor_images(image1_bytes: Vec<u8>, image2_bytes: Vec<u8>) -> PyResult<Vec<u8>> {
    let image1 = open_image_from_bytes(image1_bytes)?;
    let image2 = open_image_from_bytes(image2_bytes)?;

    let xored = pixelveil::image_steganalysis::xor_images(&image1, &image2);

    Ok(export_image_to_png_bytes(&image::DynamicImage::ImageRgb8(
        xored,
    )))
}

#[pyfunction]
fn highlight_image_difference(image1_bytes: Vec<u8>, image2_bytes: Vec<u8>) -> PyResult<Vec<u8>> {
    let image1 = open_image_from_bytes(image1_bytes)?;
    let image2 = open_image_from_bytes(image2_bytes)?;

    let highlighted = pixelveil::image_steganalysis::highlight_image_difference(&image1, &image2);

    Ok(export_image_to_png_bytes(&image::DynamicImage::ImageRgb8(
        highlighted,
    )))
}

#[pyfunction]
fn slice_image_bit_planes(image_bytes: Vec<u8>) -> PyResult<HashMap<(u8, u8), Vec<u8>>> {
    let img = open_image_from_bytes(image_bytes)?;

    let mut planes_map = pixelveil::image_steganalysis::slice_image_bit_planes(&img);

    let mut planes_bytes_map: HashMap<(u8, u8), Vec<u8>> = HashMap::with_capacity(24);

    for channel in 0..3u8 {
        for bit_index in 0..8u8 {
            planes_bytes_map.insert(
                (channel, bit_index),
                export_image_to_png_bytes(&image::DynamicImage::ImageLuma8(
                    planes_map.remove(&(channel, bit_index)).unwrap(),
                )),
            );
        }
    }

    Ok(planes_bytes_map)
}

#[pymodule(name = "image_steganalysis")]
pub(crate) fn attach_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(subtract_pixels, m)?)?;
    m.add_function(wrap_pyfunction!(xor_pixels, m)?)?;
    m.add_function(wrap_pyfunction!(subtract_images, m)?)?;
    m.add_function(wrap_pyfunction!(xor_images, m)?)?;
    m.add_function(wrap_pyfunction!(highlight_image_difference, m)?)?;
    m.add_function(wrap_pyfunction!(slice_image_bit_planes, m)?)?;

    Ok(())
}
