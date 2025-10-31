use std::io::Cursor;

use image::{DynamicImage, ImageError, ImageFormat, ImageReader};

pub(crate) fn open_image_from_raw(
    raw_data: Vec<u8>,
    format: ImageFormat,
) -> Result<DynamicImage, ImageError> {
    let cursor = Cursor::new(raw_data);
    let mut img = ImageReader::new(cursor).with_guessed_format()?;
    img.set_format(format);
    Ok(img.decode()?)
}
