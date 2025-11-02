use crate::bpcs::bit_plane::BitPlane;
use crate::utils::bit_operations::get_bits;
use image::{GenericImageView, Rgb, RgbImage};
use std::collections::HashMap;

pub(crate) const BLOCK_SIZE: u32 = 8;

pub(crate) struct Block {
    coords: (u32, u32),
    bit_planes: HashMap<(usize, usize), BitPlane>, // k: (rgb channel index, bit index)
}

impl Block {
    pub(crate) fn new(source_image: &RgbImage, coords: (u32, u32)) -> Self {
        let mut map = HashMap::new();
        for chal in 0..3 {
            for bit_index in 0..8 {
                map.insert((chal, bit_index), BitPlane::new());
            }
        }

        let mut b = Block {
            coords,
            bit_planes: map,
        };

        let img = source_image.view(coords.0, coords.1, BLOCK_SIZE, BLOCK_SIZE);
        let pixels = img.pixels();

        for (x, y, p) in pixels {
            b.insert_pixel((x as usize, y as usize), p);
        }
        b
    }

    pub(crate) fn insert_pixel(&mut self, coords: (usize, usize), pixel: Rgb<u8>) {
        for chal in 0..3 {
            let bits = get_bits(pixel.0[chal]);
            for i in 0..8 {
                if let Some(bit_plane) = self.bit_planes.get_mut(&(chal, i)) {
                    bit_plane.set(coords, bits[i as usize]);
                } else {
                    panic!(
                        "Tried to access bit plane outside of channel/bit index bounds: {chal:?},{i:?}"
                    )
                }
            }
        }
    }
}
