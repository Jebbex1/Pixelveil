//! Steganalysis functions for analyzing images.

use crate::utils::bit_operations_utils::unsigned_int_to_bits;
use image::{GrayImage, Luma, Rgb, RgbImage};
use std::collections::HashMap;

/// Subtract two 24-bit RGB pixels in each one of their channels
///
/// Uses the `Rgb` struct from the image crate to represent the value of each pixel.
/// # Example
/// ```
/// # use image::Rgb;
/// # use pixelveil::image_steganalysis::subtract_pixels;
/// let p1: Rgb<u8> = Rgb([13, 80, 40]); // R, G, B
/// let p2: Rgb<u8> = Rgb([240, 93, 31]); // R, G, B
///
/// let diff = subtract_pixels(&p1, &p2);
///
/// assert_eq!(
///     diff,
///     Rgb::<u8>([227, 13, 9]) // abs(R1 - R2), abs(G1 - G2), abs(B1 - B2)
/// );
/// ```
///
/// # Arguments
/// This function takes in two arguments:
/// * `p1: &Rgb<u8>` — The first pixel.
/// * `p2: &Rgb<u8>` — The second pixel.
///
/// # Panics
/// This function does not panic.
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// The returned value of this function is a pixel that represent the difference in each channel between the two pixels.
pub fn subtract_pixels(p1: &Rgb<u8>, p2: &Rgb<u8>) -> Rgb<u8> {
    Rgb::<u8> {
        0: [
            (p1.0[0] as i16 - p2.0[0] as i16).abs() as u8,
            (p1.0[1] as i16 - p2.0[1] as i16).abs() as u8,
            (p1.0[2] as i16 - p2.0[2] as i16).abs() as u8,
        ],
    }
}

/// XOR two 24-bit RGB pixels in each one of their channels
///
/// Uses the `Rgb` struct from the image crate to represent the value of each pixel.
/// # Example
/// ```
/// # use image::Rgb;
/// # use pixelveil::image_steganalysis::xor_pixels;
/// let p1: Rgb<u8> = Rgb([0b10110010, 0b11011100, 0b11010001]); // R, G, B
/// let p2: Rgb<u8> = Rgb([0b00100011, 0b01110001, 0b11110001]); // R, G, B
///
/// let xored = xor_pixels(&p1, &p2);
///
/// assert_eq!(
///     xored,
///     Rgb::<u8>([0b10010001, 0b10101101, 0b00100000]) // R1 ^ R2, G1 ^ G2, B1 ^ B2
/// );
/// ```
///
/// # Arguments
/// This function takes in two arguments:
/// * `p1: &Rgb<u8>` — The first pixel.
/// * `p2: &Rgb<u8>` — The second pixel.
///
/// # Panics
/// This function does not panic.
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// The returned value of this function is a pixel that is the two passed in pixels, XORed with each other.
pub fn xor_pixels(p1: &Rgb<u8>, p2: &Rgb<u8>) -> Rgb<u8> {
    Rgb::<u8> {
        0: [
            (p1.0[0] ^ p2.0[0]),
            (p1.0[1] ^ p2.0[1]),
            (p1.0[2] ^ p2.0[2]),
        ],
    }
}

/// Subtracts two 24-bit RGB images from one another
///
/// Applies the `subtract_pixels` function to each pair of pixels at the same x,y and records the result in a new image
/// of the same dimensions.
///
/// # Example
/// ```no_run
/// # use image::RgbImage;
/// # use pixelveil::image_steganalysis::subtract_images;
/// let img1 = RgbImage::new(500, 500);
/// let img2 = RgbImage::new(500, 500);
///
/// let diff = subtract_images(&img1, &img2);
/// ```
///
/// # Arguments
/// This function takes in two arguments:
/// * `image1: &RgbImage` — The first image.
/// * `image2: &RgbImage` — The second image.
///
/// # Panics
/// This function panics if the two images aren't of the same dimensions.
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// This function returns an image that every pixel at x,y is the result of applying `subtract_pixels` to the
/// corresponding pixels in the two images.
pub fn subtract_images(image1: &RgbImage, image2: &RgbImage) -> RgbImage {
    assert_eq!(image1.dimensions(), image2.dimensions());
    let mut diff_image = RgbImage::new(image1.width(), image1.height());

    for (x, y) in iproduct!(0..image1.width(), 0..image1.height()) {
        let p1 = image1.get_pixel(x, y);
        let p2 = image2.get_pixel(x, y);
        let diff = subtract_pixels(p1, p2);
        diff_image.put_pixel(x, y, diff);
    }

    diff_image
}

/// XOR two 24-bit RGB images
///
/// Applies the `xor_pixels` function to each pair of pixels at the same x,y and records the result in a new image
/// of the same dimensions.
///
/// # Example
/// ```no_run
/// # use image::RgbImage;
/// # use pixelveil::image_steganalysis::xor_images;
/// let img1 = RgbImage::new(500, 500);
/// let img2 = RgbImage::new(500, 500);
///
/// let diff = xor_images(&img1, &img2);
/// ```
///
/// # Arguments
/// This function takes in two arguments:
/// * `image1: &RgbImage` — The first image.
/// * `image2: &RgbImage` — The second image.
///
/// # Panics
/// This function panics if the two images aren't of the same dimensions.
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// This function returns an image that every pixel at x,y is the result of applying `xor_pixels` to the
/// corresponding pixels in the two images.
pub fn xor_images(image1: &RgbImage, image2: &RgbImage) -> RgbImage {
    assert_eq!(image1.dimensions(), image2.dimensions());
    let mut diff_image = RgbImage::new(image1.width(), image1.height());

    for (x, y) in iproduct!(0..image1.width(), 0..image1.height()) {
        let p1 = image1.get_pixel(x, y);
        let p2 = image2.get_pixel(x, y);
        let diff = xor_pixels(p1, p2);
        diff_image.put_pixel(x, y, diff);
    }

    diff_image
}

/// Highlight each different channel in each pixel between two 24-bit RGB images
///
/// Uses the `subtract_images` function to calculate the exact difference between two images. Then, sets every non-zero
/// value to 255.
///
/// # Example
/// ```no_run
/// # use image::RgbImage;
/// # use pixelveil::image_steganalysis::highlight_image_difference;
/// let img1 = RgbImage::new(500, 500);
/// let img2 = RgbImage::new(500, 500);
///
/// let diff = highlight_image_difference(&img1, &img2);
/// ```
///
/// # Arguments
/// This function takes in two arguments:
/// * `image1: &RgbImage` — The first image.
/// * `image2: &RgbImage` — The second image.
///
/// # Panics
/// This function panics if the two images aren't of the same dimensions.
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// This function returns an image that for each different channel in a pixel between the two images, the value of that
/// channel is 255.
pub fn highlight_image_difference(image1: &RgbImage, image2: &RgbImage) -> RgbImage {
    let mut new_image = subtract_images(image1, image2);

    for (x, y) in iproduct!(0..image1.width(), 0..image1.height()) {
        let p = new_image.get_pixel_mut(x, y);
        p.0 = {
            [
                if p.0[0] > 0 { 255 } else { 0 },
                if p.0[1] > 0 { 255 } else { 0 },
                if p.0[2] > 0 { 255 } else { 0 },
            ]
        }
    }

    new_image
}

/// Slices a 24-bit RGB image into 24 bit planes that represent each bit plane of the image as defined [here](https://en.wikipedia.org/wiki/Bit_plane)
///
/// # Example
/// ```not_run
/// # use image::RgbImage;
/// # use pixelveil::image_steganalysis::slice_image_bit_planes;
/// let img = RgbImage::new(500, 500);
///
/// let bit_planes = slice_image_bit_planes(&img);
///
/// let green_4 = bit_planes.get(&(1, 4)).unwrap(); // green channel at 4th bit index (5th bit from the left — 2^3 significance)
/// let red_7 = bit_planes.get(&(0, 7)).unwrap(); //  red channel at 7th bit index (last bit — 2^0 significance)
/// let blue_0 = bit_planes.get(&(2, 0)).unwrap(); // blue channel at 0th bit index (first bit — 2^7 significance)
/// ```
///
/// # Arguments
/// This function takes in one argument:
/// * `img: &RgbImage` — The first image.
///
/// # Panics
/// This function does not panic.
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// This function returns a `HashMap<(u8, u8), GrayImage>`.
/// Where:
/// * Each key is a tuple of the channel index (R,G,B = 0,1,2) and the bit index of a bit plane.
/// * Each value is the bit plane that corresponds to the matching key.
///
/// # Notes
/// The bit indices are ordered from left to right. This means that the most significant bit is at index 0, and the
/// least significant one is at index 7.
/// It's safe to call unwrap on the .get() function of the HashMap as long as the key has a valid channel and bit
/// index. All valid bit plane keys are guaranteed to have a value in the HashMap
pub fn slice_image_bit_planes(img: &RgbImage) -> HashMap<(u8, u8), GrayImage> {
    let mut planes_map: HashMap<(u8, u8), GrayImage> = HashMap::with_capacity(24);
    for (channel, bit_index) in iproduct!(0..3u8, 0..8u8) {
        planes_map.insert(
            (channel, bit_index),
            GrayImage::new(img.width(), img.height()),
        );
    }
    for (x, y) in iproduct!(0..img.width(), 0..img.height()) {
        let pixel = img.get_pixel(x, y);
        for channel in 0..3u8 {
            let bits = unsigned_int_to_bits(pixel.0[channel as usize]);
            for bit_index in 0..8u8 {
                planes_map
                    .get_mut(&(channel, bit_index))
                    .unwrap()
                    .put_pixel(x, y, Luma([if bits[bit_index as usize] { 255 } else { 0 }]));
            }
        }
    }

    planes_map
}
