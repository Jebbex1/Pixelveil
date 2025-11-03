use crate::bpcs::block::BLOCK_SIZE;

pub(crate) struct BitPlane {
    pub(crate) bits: [[bool; BLOCK_SIZE as usize]; BLOCK_SIZE as usize],
}

impl BitPlane {
    pub(crate) fn new() -> Self {
        BitPlane {
            bits: [[false; BLOCK_SIZE as usize]; BLOCK_SIZE as usize],
        }
    }

    pub(crate) fn set(&mut self, coords: (usize, usize), val: bool) {
        assert!(
            coords.0 < BLOCK_SIZE as usize && coords.1 < BLOCK_SIZE as usize,
            "Specified coords are out of bounds: coords: {coords:?}"
        );
        self.bits[coords.0][coords.1] = val;
    }
}

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

    #[test]
    #[should_panic(expected = "Specified coords are out of bounds: coords: (6, 9)")]
    fn test_set_out_of_bounds() {
        let mut b = BitPlane::new();
        b.set((6, 9), false);
    }
}
