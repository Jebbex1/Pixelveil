use std::io::Cursor;

use image::{
    ImageError,
    ImageFormat::{Bmp, Gif, Png},
    ImageReader, RgbImage,
};

pub(crate) fn open_lossless_image_from_raw(raw_data: Vec<u8>) -> Result<RgbImage, ImageError> {
    let cursor = Cursor::new(raw_data);
    let mut img = ImageReader::new(cursor).with_guessed_format()?;
    let format = img.format().unwrap();
    assert!(vec![Png, Bmp, Gif].contains(&format)); // make sure that the opened image is in a lossless format

    Ok(img.decode()?.to_rgb8())
}
