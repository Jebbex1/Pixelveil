pub(crate) const PLANE_SIZE: usize = 8;
const MAX_BIT_CHANGES: usize = ((PLANE_SIZE - 1) * PLANE_SIZE) + ((PLANE_SIZE - 1) * PLANE_SIZE);

pub(crate) fn checkerboard() -> [[bool; PLANE_SIZE]; PLANE_SIZE] {
    let mut board = [[false; PLANE_SIZE]; PLANE_SIZE];

    for y in 0..PLANE_SIZE {
        for x in 0..PLANE_SIZE {
            board[y][x] = (x + y) % 2 != 0;
        }
    }

    board
}

pub(crate) struct BitPlane {
    pub(crate) bits: [[bool; PLANE_SIZE]; PLANE_SIZE],
}

impl BitPlane {
    pub(crate) fn new() -> Self {
        BitPlane {
            bits: [[false; PLANE_SIZE]; PLANE_SIZE],
        }
    }

    pub(crate) fn set(&mut self, coords: (usize, usize), val: bool) {
        assert!(
            coords.0 < PLANE_SIZE as usize && coords.1 < PLANE_SIZE as usize,
            "Specified coords are out of bounds: coords: {coords:?}"
        );
        self.bits[coords.0][coords.1] = val;
    }

    pub(crate) fn conjugate(&mut self) {
        let checkerboard = checkerboard();
        for x in 0..PLANE_SIZE {
            for y in 0..PLANE_SIZE {
                self.bits[x][y] ^= checkerboard[x][y];
            }
        }
    }

    pub(crate) fn complexity_coeff(&self) -> f64 {
        let mut changes: usize = 0;
        for x in 1..PLANE_SIZE {
            for y in 0..PLANE_SIZE {
                if self.bits[x][y] != self.bits[x - 1][y] {
                    changes += 1;
                }
            }
        }
        for y in 1..PLANE_SIZE {
            for x in 0..PLANE_SIZE {
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
    use crate::bpcs::bit_plane::{BitPlane, PLANE_SIZE, checkerboard};

    #[test]
    fn test_creation() {
        let b = BitPlane::new();
        assert_eq!(b.bits, [[false; PLANE_SIZE]; PLANE_SIZE]);
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

    #[test]
    fn test_conjugation() {
        let mut expected = checkerboard();
        expected[0][0] = true;

        let mut p = BitPlane::new();
        p.set((0, 0), true);

        p.conjugate();

        assert_eq!(p.bits, expected);
    }

    #[test]
    fn test_complexity_coeff_calc() {
        let b1 = BitPlane {
            bits: [[false; PLANE_SIZE]; PLANE_SIZE],
        };
        assert_eq!(b1.complexity_coeff(), 0f64);

        let b2 = BitPlane {
            bits: [[true; PLANE_SIZE]; PLANE_SIZE],
        };
        assert_eq!(b2.complexity_coeff(), 0f64);

        let b3 = BitPlane {
            bits: checkerboard(),
        };
        assert_eq!(b3.complexity_coeff(), 1f64);
    }
}
