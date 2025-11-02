use crate::bpcs::bit_plane::BitPlane;
use crate::utils::bit_operations::get_bits;
use image::{GenericImageView, Rgb, RgbImage, SubImage};

pub(crate) const BLOCK_SIZE: u32 = 8;

struct Block<'a> {
    coords: (u32, u32),
    sub_image: SubImage<&'a RgbImage>,
    // TODO: make a way of storing all the 24 bit planes in the block, maybe using a HashMap with the channel and bit index as the keys.
}

impl<'a> Block<'a> {
    fn new(source_image: &'a RgbImage, coords: (u32, u32)) -> Self {
        Block {
            coords,
            sub_image: source_image.view(coords.0, coords.1, BLOCK_SIZE, BLOCK_SIZE),
        }
    }

    fn insert_pixel(&mut self, coords: (u32, u32), pixel: Rgb<u8>) {
        for byte in pixel.0 {
            // TODO: track the current channel of the pixel and use it with the bit index to insert the bit into the matching bit plane.
            let bits = get_bits(byte);
        }
    }
}
