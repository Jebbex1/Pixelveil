use crate::utils::bit_operations::{u8_to_binary_code, u8_to_gray_code};
use image::{
    ImageError,
    ImageReader, Rgb, RgbImage, open,
};
use std::io::Cursor;

pub fn open_rgbimage_from_raw(raw_data: Vec<u8>) -> Result<RgbImage, ImageError> {
    let cursor = Cursor::new(raw_data);
    let img = ImageReader::new(cursor).with_guessed_format()?;

    Ok(img.decode()?.to_rgb8())
}

pub fn open_rgbimage_from_path(path: &str) -> Result<RgbImage, ImageError> {
    Ok(open(path)?.to_rgb8())
}

pub(crate) fn pixel_to_gray_code(pixel: &mut Rgb<u8>) {
    pixel.0[0] = u8_to_gray_code(pixel.0[0]);
    pixel.0[1] = u8_to_gray_code(pixel.0[1]);
    pixel.0[2] = u8_to_gray_code(pixel.0[2]);
}

pub(crate) fn pixel_to_binary_code(pixel: &mut Rgb<u8>) {
    pixel.0[0] = u8_to_binary_code(pixel.0[0]);
    pixel.0[1] = u8_to_binary_code(pixel.0[1]);
    pixel.0[2] = u8_to_binary_code(pixel.0[2]);
}

pub fn image_to_gray_code(image: &mut RgbImage) {
    for pixel in image.pixels_mut() {
        pixel_to_gray_code(pixel);
    }
}

pub fn image_to_binary_code(image: &mut RgbImage) {
    for pixel in image.pixels_mut() {
        pixel_to_binary_code(pixel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_to_gray_code() {
        let mut pixel = Rgb([0b11101010u8, 0b00110001u8, 0b10101110u8]);
        pixel_to_gray_code(&mut pixel);
        assert_eq!(pixel, Rgb([0b10011111u8, 0b00101001u8, 0b11111001u8,]),);
    }

    #[test]
    fn test_pixel_to_binary_code() {
        let mut pixel = Rgb([0b10011111u8, 0b00101001u8, 0b11111001u8]);
        pixel_to_binary_code(&mut pixel);
        assert_eq!(pixel, Rgb([0b11101010u8, 0b00110001u8, 0b10101110u8,]),);
    }
}
