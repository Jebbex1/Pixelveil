//! Utility functions that help reading images, and converting them to [Gray Code](https://en.wikipedia.org/wiki/Gray_code) and back

use crate::utils::bit_operations_utils::{u8_to_binary_code, u8_to_gray_code};
use image::{ImageError, ImageFormat, ImageReader, Rgb, RgbImage, open};
use std::io::Cursor;

/// Open image from the raw data that describes an image file
///
/// # Example
/// ```no_run
/// # use pixelveil::image_utils::open_rgbimage_from_raw;
/// use std::fs::read;
///
/// let image_file_data = read("example.png")?;
/// let img = open_rgbimage_from_raw(image_file_data)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Arguments
/// This function takes in one argument:
/// * `raw_data: Vec<u8>` — The image file data.
///
/// # Panics
/// This function does not panic.
///
/// # Errors
/// The possible errors that can be returned are:
/// * `ImageError` if the data that was passed in is not a valid image file format
///
/// # Returns
/// This function returns a `Result<RgbImage, ImageError>`.
/// If `Ok` is returned, the unwrapped value is the opened `RgbImage`.
pub fn open_rgbimage_from_raw(raw_data: Vec<u8>) -> Result<RgbImage, ImageError> {
    let cursor = Cursor::new(raw_data);
    let img = ImageReader::new(cursor).with_guessed_format()?;

    Ok(img.decode()?.to_rgb8())
}

/// Open image from a path
///
/// # Example
/// ```no_run
/// # use pixelveil::image_utils::open_rgbimage_from_path;
/// let img = open_rgbimage_from_path("image.png")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Arguments
/// This function takes in one argument:
/// * `path: &str` — The image path.
///
/// # Panics
/// This function does not panic.
///
/// # Errors
/// The possible errors that can be returned are:
/// * `ImageError` if the path specifies a missing file or one of invalid image format
///
/// # Returns
/// This function returns a `Result<RgbImage, ImageError>`.
/// If `Ok` is returned, the unwrapped value is the opened `RgbImage`.
pub fn open_rgbimage_from_path(path: &str) -> Result<RgbImage, ImageError> {
    Ok(open(path)?.to_rgb8())
}

/// Converts a 24-bit RGB pixel from pure binary code to Gray Code as defined [here](https://en.wikipedia.org/wiki/Gray_code)
///
/// # Example
/// ```
/// # use pixelveil::image_utils::pixel_to_gray_code;
/// # use image::Rgb;
/// let mut pixel = Rgb::<u8>([0b1110101, 0b0011000, 0b1010111]);
///
/// pixel_to_gray_code(&mut pixel);
///
/// assert_eq!(pixel, Rgb::<u8>([0b1001111, 0b0010100, 0b1111100]));
/// ```
///
/// # Arguments
/// This function takes in one argument:
/// * `pixel: &mut Rgb<u8>` — The pixel to convert.
///
/// # Panics
/// This function does not panic.
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// This function returns (), the passed in pixel will be changed instead of constructing a new pixel.
pub fn pixel_to_gray_code(pixel: &mut Rgb<u8>) {
    pixel.0[0] = u8_to_gray_code(pixel.0[0]);
    pixel.0[1] = u8_to_gray_code(pixel.0[1]);
    pixel.0[2] = u8_to_gray_code(pixel.0[2]);
}

/// Converts a 24-bit RGB pixel from Gray Code to pure binary code as defined [here](https://en.wikipedia.org/wiki/Gray_code)
///
/// # Example
/// ```
/// # use pixelveil::image_utils::pixel_to_binary_code;
/// # use image::Rgb;
/// let mut pixel = Rgb::<u8>([0b10011111, 0b00101001, 0b11111001]);
///
/// pixel_to_binary_code(&mut pixel);
///
/// assert_eq!(pixel, Rgb::<u8>([0b11101010, 0b00110001, 0b10101110]));
/// ```
///
/// # Arguments
/// This function takes in one argument:
/// * `pixel: &mut Rgb<u8>` — The pixel to convert.
///
/// # Panics
/// This function does not panic.
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// This function returns (), the passed in pixel will be changed instead of constructing a new pixel.
pub fn pixel_to_binary_code(pixel: &mut Rgb<u8>) {
    pixel.0[0] = u8_to_binary_code(pixel.0[0]);
    pixel.0[1] = u8_to_binary_code(pixel.0[1]);
    pixel.0[2] = u8_to_binary_code(pixel.0[2]);
}

/// Converts an image from pure binary code to Gray Code
///
/// Applies `pixel_to_gray_code` on every pixel in the image.
///
/// # Example
/// ```no_run
/// # use image::RgbImage;
/// # use pixelveil::image_utils::image_to_gray_code;
/// let mut img = RgbImage::new(500, 500);
///
/// image_to_gray_code(&mut img);
/// ```
///
/// # Arguments
/// This function takes in one argument:
/// * `image: &mut RgbImage` — The image to convert.
///
/// # Panics
/// This function does not panic.
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// This function returns (), the passed in image will be changed instead of constructing a new image.
pub fn image_to_gray_code(image: &mut RgbImage) {
    for pixel in image.pixels_mut() {
        pixel_to_gray_code(pixel);
    }
}

/// Converts an image from Gray Code to pure binary code
///
/// Applies `pixel_to_binary_code` on every pixel in the image.
///
/// # Example
/// ```no_run
/// # use image::RgbImage;
/// # use pixelveil::image_utils::image_to_binary_code;
/// let mut img = RgbImage::new(500, 500);
///
/// image_to_binary_code(&mut img);
/// ```
///
/// # Arguments
/// This function takes in one argument:
/// * `image: &mut RgbImage` — The image to convert.
///
/// # Panics
/// This function does not panic.
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// This function returns (), the passed in image will be changed instead of constructing a new image.
pub fn image_to_binary_code(image: &mut RgbImage) {
    for pixel in image.pixels_mut() {
        pixel_to_binary_code(pixel);
    }
}

/// Export an RgbImage to a .png file format
///
/// # Example
/// ```no_run
/// # use pixelveil::image_utils::export_image_to_png_bytes;
/// # use image::RgbImage;
/// let image = RgbImage::new(500, 500);
/// let image_file_bytes = export_image_to_png_bytes(&image);
/// ```
///
/// # Arguments
/// This function takes in one argument:
/// * `img: &RgbImage` — The image you want to export.
///
/// # Panics
/// This function does not panic.
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// This function returns a `Vec<u8>` that describes the file bytes in the .png file format
pub fn export_image_to_png_bytes(img: &RgbImage) -> Vec<u8> {
    // Create an in-memory buffer (Vec<u8>) wrapped in a Cursor
    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);

    // Use the write_to method to encode the image into the cursor/buffer
    img.write_to(&mut cursor, ImageFormat::Png).unwrap();

    // Return the resulting bytes
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_to_gray_code() {
        let mut pixel = Rgb::<u8>([0b1110101, 0b0011000, 0b1010111]);
        pixel_to_gray_code(&mut pixel);
        assert_eq!(pixel, Rgb::<u8>([0b1001111, 0b0010100, 0b1111100]));
    }

    #[test]
    fn test_pixel_to_binary_code() {
        let mut pixel = Rgb::<u8>([0b10011111, 0b00101001, 0b11111001]);
        pixel_to_binary_code(&mut pixel);
        assert_eq!(pixel, Rgb::<u8>([0b11101010, 0b00110001, 0b10101110]));
    }
}
