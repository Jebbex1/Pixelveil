use std::{
    iter,
    str::{self, Utf8Error},
};

pub(crate) fn xor_bits(
    a: impl Iterator<Item = bool>,
    b: impl Iterator<Item = bool>,
) -> impl Iterator<Item = bool> {
    a.zip(b).map(|(x, y)| x ^ y)
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
}
