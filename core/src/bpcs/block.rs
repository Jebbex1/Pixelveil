use crate::bpcs::bit_plane::BitPlane;
use crate::utils::bit_operations::get_bits;
use image::{GenericImageView, Rgb, RgbImage, SubImage};
use std::collections::HashMap;

pub(crate) const BLOCK_SIZE: u32 = 8;

pub(crate) struct Block {
    pub(crate) coords: (u32, u32),
    pub(crate) bit_planes: HashMap<(usize, usize), BitPlane>, // k: (rgb channel index, bit index)
}

impl Block {
    pub(crate) fn new(source_sub_image: &SubImage<&RgbImage>) -> Self {
        assert_eq!(
            source_sub_image.dimensions(),
            (BLOCK_SIZE, BLOCK_SIZE),
            "Source sub image for block does not have the correct dimensions."
        );

        let mut map = HashMap::new();
        for chal in 0..3 {
            for bit_index in 0..8 {
                map.insert((chal, bit_index), BitPlane::new());
            }
        }

        let mut b = Block {
            coords: source_sub_image.offsets(),
            bit_planes: map,
        };

        let pixels = source_sub_image.pixels();

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

#[cfg(test)]
mod tests {
    use image::{GenericImageView, Rgb};
    use std::fs;

    use crate::{
        bpcs::block::{BLOCK_SIZE, Block},
        utils::image_handling::open_lossless_image_from_raw,
    };

    fn get_new_empty_block() -> Result<Block, Box<dyn std::error::Error>> {
        let bytes = fs::read("tests/assets/empty_block.png")?;
        let image = open_lossless_image_from_raw(bytes)?;

        Ok(Block::new(&image.view(0, 0, 8, 8)))
    }

    #[test]
    fn test_new() -> Result<(), Box<dyn std::error::Error>> {
        let b = get_new_empty_block()?;

        for chal in 0..3 as usize {
            for bit_index in 0..8 as usize {
                let p = b.bit_planes.get(&(chal, bit_index)).unwrap();
                assert_eq!(p.bits, [[false; BLOCK_SIZE as usize]; BLOCK_SIZE as usize]);
            }
        }

        Ok(())
    }

    #[test]
    #[should_panic(expected = "Source sub image for block does not have the correct dimensions.")]
    fn test_new_incorrect_sub_image_size() {
        let bytes = fs::read("tests/assets/empty_block.png").unwrap();
        let image = open_lossless_image_from_raw(bytes).unwrap();

        Block::new(&image.view(0, 0, 6, 8));
    }

    #[test]
    fn test_insert_pixel() -> Result<(), Box<dyn std::error::Error>> {
        let mut b = get_new_empty_block()?;
        let mut bits_expected = [[false; BLOCK_SIZE as usize]; BLOCK_SIZE as usize];
        bits_expected[1][1] = true;

        b.insert_pixel((1, 1), Rgb::<u8> { 0: [255, 255, 255] });

        for chal in 0..3 as usize {
            for bit_index in 0..8 as usize {
                let p = b.bit_planes.get(&(chal, bit_index)).unwrap();
                assert_eq!(p.bits, bits_expected);
            }
        }

        Ok(())
    }
}
