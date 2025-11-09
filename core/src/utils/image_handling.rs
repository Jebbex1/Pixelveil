use crate::utils::bit_operations::{to_binary_code, to_gray_code};
use image::{
    ImageError,
    ImageFormat::{Bmp, Gif, Png},
    ImageReader, Rgb, RgbImage,
};
use std::io::Cursor;

pub(crate) fn open_lossless_image_from_raw(raw_data: Vec<u8>) -> Result<RgbImage, ImageError> {
    let cursor = Cursor::new(raw_data);
    let img = ImageReader::new(cursor).with_guessed_format()?;
    let format = img.format().unwrap();
    assert!(vec![Png, Bmp, Gif].contains(&format)); // make sure that the opened image is in a lossless format

    Ok(img.decode()?.to_rgb8())
}

pub(crate) fn pixel_to_gray_code(pixel: Rgb<u8>) -> Rgb<u8> {
    Rgb([
        to_gray_code(pixel.0[0]),
        to_gray_code(pixel.0[1]),
        to_gray_code(pixel.0[2]),
    ])
}

pub(crate) fn pixel_to_binary_code(pixel: Rgb<u8>) -> Rgb<u8> {
    Rgb([
        to_binary_code(pixel.0[0]),
        to_binary_code(pixel.0[1]),
        to_binary_code(pixel.0[2]),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_to_gray_code() {
        assert_eq!(
            Rgb([
                to_gray_code(0b11101010u8),
                to_gray_code(0b00110001u8),
                to_gray_code(0b10101110u8),
            ]),
            Rgb([0b10011111u8, 0b00101001u8, 0b11111001u8,]),
        );
    }

    #[test]
    fn test_pixel_to_binary_code() {
        assert_eq!(
            Rgb([0b11101010u8, 0b00110001u8, 0b10101110u8,]),
            Rgb([
                to_binary_code(0b10011111u8),
                to_binary_code(0b00101001u8),
                to_binary_code(0b11111001u8),
            ]),
        );
    }
}
