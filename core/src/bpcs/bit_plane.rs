use crate::utils::bit_operations::{bits_to_byte, get_bit};
use image::{GenericImageView, RgbImage, SubImage};

pub(crate) const PLANE_SIZE: u32 = 8;
pub(crate) const USIZE_PLANE_SIZE: usize = PLANE_SIZE as usize;
pub(crate) const BYTES_PER_PLANE: usize = (USIZE_PLANE_SIZE * USIZE_PLANE_SIZE) / 8;
const MAX_BIT_CHANGES: u32 = ((PLANE_SIZE - 1) * PLANE_SIZE) + ((PLANE_SIZE - 1) * PLANE_SIZE);

pub(crate) fn checkerboard() -> [[bool; USIZE_PLANE_SIZE]; USIZE_PLANE_SIZE] {
    let mut board = [[false; USIZE_PLANE_SIZE]; USIZE_PLANE_SIZE];

    for y in 0..USIZE_PLANE_SIZE {
        for x in 0..USIZE_PLANE_SIZE {
            board[y][x] = (x + y) % 2 != 0;
        }
    }

    board
}

fn get_planes(mut bits: Vec<bool>) -> Vec<BitPlane> {
    assert!(
        bits.len() % (USIZE_PLANE_SIZE * USIZE_PLANE_SIZE) == 0,
        "Tried to construct bit planes with remaining bits that don't fill up an entire plane."
    );
    let mut planes: Vec<BitPlane> = Vec::new();
    while !bits.is_empty() {
        let plane_bits: Vec<bool> = bits
            .drain(0..(USIZE_PLANE_SIZE * USIZE_PLANE_SIZE))
            .collect();
        planes.push(BitPlane::from_bits(plane_bits.try_into().unwrap()));
    }
    planes
}

#[derive(Debug)]
pub(crate) struct BitPlane {
    pub(crate) bits: [[bool; USIZE_PLANE_SIZE]; USIZE_PLANE_SIZE],
}

impl BitPlane {
    pub(crate) fn new() -> Self {
        BitPlane {
            bits: [[false; USIZE_PLANE_SIZE]; USIZE_PLANE_SIZE],
        }
    }

    pub(crate) fn from_sub_image(
        sub_image: SubImage<&RgbImage>,
        channel: u8,
        bit_index: u8,
    ) -> Self {
        assert!(
            sub_image.width() == PLANE_SIZE && sub_image.height() == PLANE_SIZE,
            "Supplied SubImage incorrect dimensions to block of dimensions {PLANE_SIZE},{PLANE_SIZE}."
        );
        let mut p = BitPlane::new();
        for (x, y, pixel) in sub_image.pixels() {
            p.set_bit(
                (x as usize, y as usize),
                get_bit(pixel.0[channel as usize], bit_index),
            );
        }
        p
    }

    pub(crate) fn from_bits(bit_array: [bool; USIZE_PLANE_SIZE * USIZE_PLANE_SIZE]) -> Self {
        let mut p = BitPlane::new();
        for i in 0..USIZE_PLANE_SIZE {
            for j in 0..USIZE_PLANE_SIZE {
                p.set_bit((i, j), bit_array[(i * USIZE_PLANE_SIZE) + j]);
            }
        }
        p
    }

    pub(crate) fn export_to_bools(self) -> [bool; USIZE_PLANE_SIZE * USIZE_PLANE_SIZE] {
        let bits_flattened: [bool; USIZE_PLANE_SIZE * USIZE_PLANE_SIZE] = self
            .bits
            .into_iter()
            .flatten()
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap();
        bits_flattened
    }

    pub(crate) fn export_to_u8s(self) -> [u8; BYTES_PER_PLANE] {
        let bits_flattened: [bool; USIZE_PLANE_SIZE * USIZE_PLANE_SIZE] = self.export_to_bools();
        let mut bytes = [0u8; BYTES_PER_PLANE];
        for i in 0..BYTES_PER_PLANE {
            bytes[i] = bits_to_byte(bits_flattened[i * 8..(i + 1) * 8].try_into().unwrap())
        }
        bytes
    }

    pub(crate) fn set_bit(&mut self, coords: (usize, usize), val: bool) {
        assert!(
            coords.0 < PLANE_SIZE as usize && coords.1 < PLANE_SIZE as usize,
            "Specified coords are out of bounds: coords: {coords:?}"
        );
        self.bits[coords.0][coords.1] = val;
    }

    pub(crate) fn conjugate(&mut self) {
        let checkerboard = checkerboard();
        for x in 0..USIZE_PLANE_SIZE {
            for y in 0..USIZE_PLANE_SIZE {
                self.bits[x][y] ^= checkerboard[x][y];
            }
        }
    }

    pub(crate) fn alpha(&self) -> f64 {
        let mut changes: usize = 0;
        for x in 1..USIZE_PLANE_SIZE {
            for y in 0..USIZE_PLANE_SIZE {
                if self.bits[x][y] != self.bits[x - 1][y] {
                    changes += 1;
                }
            }
        }
        for y in 1..USIZE_PLANE_SIZE {
            for x in 0..USIZE_PLANE_SIZE {
                if self.bits[x][y] != self.bits[x][y - 1] {
                    changes += 1;
                }
            }
        }
        (changes as f64) / (MAX_BIT_CHANGES as f64)
    }
}

#[cfg(test)]
mod tests {
    use crate::bpcs::bit_plane::{
        BitPlane, PLANE_SIZE, USIZE_PLANE_SIZE, checkerboard, get_planes,
    };
    use image::GenericImageView;

    #[test]
    fn test_creation() {
        let b = BitPlane::new();
        assert_eq!(b.bits, [[false; USIZE_PLANE_SIZE]; USIZE_PLANE_SIZE]);
    }

    #[test]
    fn test_set_bit() {
        let mut b = BitPlane::new();
        b.set_bit((0, 0), true);
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
        b.set_bit((6, 9), false);
    }

    #[test]
    fn test_conjugation() {
        let mut expected = checkerboard();
        expected[0][0] = true;

        let mut p = BitPlane::new();
        p.set_bit((0, 0), true);

        p.conjugate();

        assert_eq!(p.bits, expected);
    }

    #[test]
    fn test_complexity_coeff_calc() {
        let b1 = BitPlane {
            bits: [[false; USIZE_PLANE_SIZE]; USIZE_PLANE_SIZE],
        };
        assert_eq!(b1.alpha(), 0f64);

        let b2 = BitPlane {
            bits: [[true; USIZE_PLANE_SIZE]; USIZE_PLANE_SIZE],
        };
        assert_eq!(b2.alpha(), 0f64);

        let b3 = BitPlane {
            bits: checkerboard(),
        };
        assert_eq!(b3.alpha(), 1f64);
    }

    #[test]
    fn test_from_sub_image() -> Result<(), Box<dyn std::error::Error>> {
        let img = image::open("tests/assets/test_img_1.png")?.to_rgb8();
        let sub_img = img.view(0, 0, PLANE_SIZE as u32, PLANE_SIZE as u32);
        let p = BitPlane::from_sub_image(sub_img, 1, 1);
        assert_eq!(p.bits, [[true; USIZE_PLANE_SIZE]; USIZE_PLANE_SIZE]);
        Ok(())
    }

    #[test]
    fn test_from_bits() {
        let mut bits = [false; USIZE_PLANE_SIZE * USIZE_PLANE_SIZE];
        bits[1] = true;
        bits[10] = true;

        let mut expected = [[false; USIZE_PLANE_SIZE]; USIZE_PLANE_SIZE];
        expected[0][1] = true;
        expected[1][2] = true;

        let p = BitPlane::from_bits(bits);
        assert_eq!(p.bits, expected);
    }

    #[test]
    fn test_export_to_u8s() {
        let mut bits = [false; USIZE_PLANE_SIZE * USIZE_PLANE_SIZE];
        bits[1] = true;
        bits[10] = true;

        let p = BitPlane::from_bits(bits);
        let bytes = p.export_to_u8s();

        assert_eq!(
            bytes,
            [
                0b01000000u8,
                0b00100000u8,
                0b00000000u8,
                0b00000000u8,
                0b00000000u8,
                0b00000000u8,
                0b00000000u8,
                0b00000000u8
            ]
        );
    }

    #[test]
    fn test_get_planes() {
        let block1 = vec![
            false, true, false, false, false, false, true, false, false, false, false, false, true,
            false, false, false, false, false, false, false, false, false, false, true, false,
            true, false, false, false, true, false, false, false, false, false, false, false,
            false, true, false, false, false, false, true, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false, false, false, false, true,
            false, false,
        ];
        let block2 = vec![
            false, true, false, false, false, true, true, false, false, false, false, false, false,
            false, false, false, true, false, false, false, true, false, false, true, false, false,
            false, true, false, false, false, false, true, false, true, false, false, true, false,
            false, true, false, false, false, false, false, true, false, false, true, false, false,
            true, false, false, true, false, false, false, false, false, false, true, false,
        ];

        let mut planes: Vec<BitPlane> = get_planes([block1.as_slice(), block2.as_slice()].concat())
            .into_iter()
            .collect();
        assert_eq!(planes.remove(0).export_to_bools().to_vec(), block1);
        assert_eq!(planes.remove(0).export_to_bools().to_vec(), block2);
    }
}
