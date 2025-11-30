use crate::utils::bit_operations_utils::unsigned_int_to_bits;
use image::{GrayImage, Luma, Rgb, RgbImage};
use std::collections::HashMap;

pub fn subtract_pixels(p1: &Rgb<u8>, p2: &Rgb<u8>) -> Rgb<u8> {
    Rgb::<u8> {
        0: [
            (p1.0[0] as i16 - p2.0[0] as i16).abs() as u8,
            (p1.0[1] as i16 - p2.0[1] as i16).abs() as u8,
            (p1.0[2] as i16 - p2.0[2] as i16).abs() as u8,
        ],
    }
}

pub fn xor_pixels(p1: &Rgb<u8>, p2: &Rgb<u8>) -> Rgb<u8> {
    Rgb::<u8> {
        0: [
            (p1.0[0] ^ p2.0[0]),
            (p1.0[1] ^ p2.0[1]),
            (p1.0[2] ^ p2.0[2]),
        ],
    }
}

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
