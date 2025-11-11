use std::{
    iter,
    ops::{BitAnd, ShrAssign},
    str::{self, Utf8Error},
};

use num::{One, PrimInt, Unsigned, Zero};

pub(crate) fn xor_bits(
    a: impl Iterator<Item = bool>,
    b: impl Iterator<Item = bool>,
) -> impl Iterator<Item = bool> {
    a.zip(b).map(|(x, y)| x ^ y)
}

pub(crate) fn bits_to_u8(bits: [bool; 8]) -> u8 {
    let mut byte = 0u8;
    for i in 0..8 {
        byte <<= 1;
        if bits[i] {
            byte |= 1;
        }
    }
    byte
}

pub(crate) fn bits_to_u64(bits: [bool; 64]) -> u64 {
    let mut num = 0u64;
    for i in 0..64 {
        num <<= 1;
        if bits[i] {
            num |= 1;
        }
    }
    num
}

pub(crate) fn unsigned_int_to_bits<T>(mut num: T) -> Vec<bool>
where
    T: Unsigned + PrimInt + PartialEq + BitAnd + ShrAssign + Zero + One,
{
    let bit_depth = std::mem::size_of::<T>() * 8;
    let mut v: Vec<bool> = Vec::with_capacity(bit_depth);
    while num != T::zero() {
        let bit = (num & T::one()) == T::one();
        v.insert(0, bit);
        num >>= T::one();
    }

    while v.len() != bit_depth {
        v.insert(0, false);
    }

    v
}

pub(crate) fn bits_to_bytes(mut bits: impl Iterator<Item = bool>) -> impl Iterator<Item = u8> {
    iter::from_fn(move || {
        let mut byte = 0u8;
        let mut bits_read = 0;

        for _ in 0..8 {
            match bits.next() {
                Some(bit) => {
                    byte <<= 1;
                    if bit {
                        byte |= 1;
                    }
                    bits_read += 1;
                }
                None => break,
            }
        }

        if bits_read == 0 { None } else { Some(byte) }
    })
}

pub(crate) fn bytes_to_utf8(v: &Vec<u8>) -> Result<&str, Utf8Error> {
    str::from_utf8(v)
}

pub(crate) fn get_bit_from_u8(byte: u8, bit_index: u8) -> bool {
    assert!(bit_index < 8);
    ((byte >> 7 - bit_index) & 1) != 0
}

pub(crate) fn u8_to_gray_code(byte: u8) -> u8 {
    byte ^ (byte >> 1)
}

pub(crate) fn u8_to_binary_code(byte: u8) -> u8 {
    let mut mask = byte;
    let mut binary = byte;
    while mask != 0 {
        mask >>= 1;
        binary ^= mask;
    }
    binary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_vecs() {
        let a = vec![true, true, false, false].into_iter();
        let b = vec![true, false, true, false].into_iter();
        let c: Vec<bool> = xor_bits(a, b).collect();

        assert_eq!(c, vec![false, true, true, false]);
    }

    #[test]
    fn test_bits_to_u8s() {
        let bits = vec![true, false, true, true, false, false, false, true, true];
        let packed: Vec<u8> = bits_to_bytes(bits.into_iter()).collect();

        assert_eq!(packed, vec![0b10110001, 0b1])
    }

    #[test]
    fn test_bytes_to_str() {
        let v1 = vec![0x20, 0x00];
        assert_eq!(bytes_to_utf8(&v1).unwrap(), "\x20\x00");
    }

    #[test]
    fn test_get_bit() {
        let b1 = 0b01000000 as u8;
        assert_eq!(get_bit_from_u8(b1, 1), true)
    }

    #[test]
    fn test_to_gray_code() {
        assert_eq!(0b0010u8, u8_to_gray_code(0b0011u8)); // 3u8
        assert_eq!(0b1100u8, u8_to_gray_code(0b1000u8)); // 8u8
        assert_eq!(0b1011u8, u8_to_gray_code(0b1101u8)); // 13u8
        assert_eq!(0b1000u8, u8_to_gray_code(0b1111u8)); // 15u8
    }

    #[test]
    fn test_to_binary_code() {
        assert_eq!(u8_to_binary_code(0b0010u8), 0b0011u8); // 3u8
        assert_eq!(u8_to_binary_code(0b1100u8), 0b1000u8); // 8u8
        assert_eq!(u8_to_binary_code(0b1011u8), 0b1101u8); // 13u8
        assert_eq!(u8_to_binary_code(0b1000u8), 0b1111u8); // 15u8
    }

    #[test]
    fn test_generic_unsigned_int_to_bits() {
        let num: u8 = 235;
        let bits: [bool; 8] = unsigned_int_to_bits(num).try_into().unwrap();
        let expected = [true, true, true, false, true, false, true, true];
        assert_eq!(bits, expected);

        let num: u16 = 5274;
        let bits: [bool; 16] = unsigned_int_to_bits(num).try_into().unwrap();
        let expected = [
            false, false, false, true, false, true, false, false, true, false, false, true, true,
            false, true, false,
        ];
        assert_eq!(bits, expected);

        let num: u32 = 1583765927;
        let bits: [bool; 32] = unsigned_int_to_bits(num).try_into().unwrap();
        let expected = [
            false, true, false, true, true, true, true, false, false, true, true, false, false,
            true, true, false, false, true, false, true, true, false, false, true, true, false,
            true, false, false, true, true, true,
        ];
        assert_eq!(bits, expected);

        let num: u64 = 837659277593756383;
        let bits: [bool; 64] = unsigned_int_to_bits(num).try_into().unwrap();
        let expected = [
            false, false, false, false, true, false, true, true, true, false, false, true, true,
            true, true, true, true, true, true, true, false, true, true, false, true, false, true,
            false, true, true, false, false, true, true, false, false, false, false, true, true,
            true, true, false, false, true, false, false, true, false, true, false, false, false,
            false, true, false, true, true, false, true, true, true, true, true,
        ];
        assert_eq!(bits, expected);
    }
}
