use crate::bpcs::block::BLOCK_SIZE;

pub(crate) struct BitPlane {
    bits: [[bool; BLOCK_SIZE as usize]; BLOCK_SIZE as usize],
}

impl BitPlane {
    pub(crate) fn new() -> Self {
        BitPlane {
            bits: [[false; BLOCK_SIZE as usize]; BLOCK_SIZE as usize],
        }
    }

    pub(crate) fn set(&mut self, coords: (usize, usize), val: bool) {
        self.bits[coords.0][coords.1] = val;
    }
}

/*
impl BitPlane {
    fn new(coords: (u32, u32), channel_index: u8, bit_index: u8) -> Self {
        BitPlane {
            coords: coords,
            channel_index: channel_index,
            bit_index: bit_index,
            bits: [[false; BLOCK_SIZE as usize]; BLOCK_SIZE as usize],
        }
    }
    fn from_block(
        block: SubImage<&RgbImage>,
        coords: (u32, u32),
        channel_index: u8,
        bit_index: u8,
    ) -> Self {
        let pixels = block.pixels();
        for pixel in pixels {
            println!("{pixel:?}");
        }
        // TODO: help
        let plane = [[false; BLOCK_SIZE as usize]; BLOCK_SIZE as usize];

        BitPlane {
            coords: coords,
            channel_index: channel_index,
            bit_index: bit_index,
            bits: plane,
        }
    }
}

struct BitPlaneIter<'a> {
    source_image: &'a RgbImage,

    curr_width: u32,
    curr_height: u32,
    curr_channel_index: u8,
    curr_bit_index: u8,
}

impl<'a> BitPlaneIter<'a> {
    fn new(source_image: &'a RgbImage) -> Self {
        BitPlaneIter {
            source_image: &source_image,

            curr_width: 0,
            curr_height: 0,
            curr_channel_index: 0,
            curr_bit_index: 7, // start with the LSB, then gradually move to the MSB
        }
    }

    fn increment(&mut self) -> Result<(), ()> {
        if self.curr_width < self.source_image.width() - 1 - BLOCK_SIZE {
            self.curr_width += BLOCK_SIZE;
            return Ok(());
        }
        self.curr_width = 0;

        if self.curr_height < self.source_image.height() - 1 - BLOCK_SIZE {
            self.curr_height += BLOCK_SIZE;
            return Ok(());
        }
        self.curr_height = 0;

        if self.curr_channel_index < 3 - 1 {
            self.curr_channel_index += 1;
            return Ok(());
        }
        self.curr_channel_index = 0;

        if self.curr_bit_index > 0 {
            self.curr_bit_index -= 1;
            return Ok(());
        }
        self.curr_bit_index = 0;

        Err(())
    }
}

impl<'a> Iterator for BitPlaneIter<'a> {
    type Item = BitPlane;
    fn next(&mut self) -> Option<Self::Item> {
        let sub_image =
            self.source_image
                .view(self.curr_width, self.curr_height, BLOCK_SIZE, BLOCK_SIZE);

        Some(BitPlane::from_block(
            sub_image,
            (self.curr_width, self.curr_height),
            self.curr_channel_index,
            self.curr_bit_index,
        ))
    }
}
*/
#[cfg(test)]
mod tests {
    use crate::bpcs::bit_plane::BitPlane;
    use crate::bpcs::block::BLOCK_SIZE;

    #[test]
    fn test_creation() {
        let b = BitPlane::new();
        assert_eq!(b.bits, [[false; BLOCK_SIZE as usize]; BLOCK_SIZE as usize]);
    }

    #[test]
    fn test_set_bit() {
        let mut b = BitPlane::new();
        b.set((0, 0), true);
        assert_eq!(
            b.bits,
            [
                [true, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
            ]
        )
    }
}
