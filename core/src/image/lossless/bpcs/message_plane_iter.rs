use rand::{Rng, rng};

use crate::{
    image::lossless::bpcs::bit_plane::{BYTES_PER_PLANE, BitPlane},
    utils::bit_operations::unsigned_int_to_bits,
};

pub(crate) fn get_bytes_per_plane_u8s<'a, T>(iter: &'a mut T) -> Option<(Vec<u8>, usize)>
where
    T: Iterator<Item = u8>,
{
    let mut u8s: Vec<u8> = Vec::with_capacity(BYTES_PER_PLANE);
    let mut num_of_data_bytes = 0usize;

    for _ in 0..BYTES_PER_PLANE {
        // if the next item is found, push it.
        // if not, check if the u8s vec contains any data, if it does, fill it with random data
        // if not, return None
        if let Some(next) = iter.next() {
            u8s.push(next);
            num_of_data_bytes += 1;
        } else {
            if u8s.is_empty() {
                return None;
            } else {
                u8s.push(rng().random_range(0u8..0b11111111u8));
            }
        }
    }

    Some((u8s, num_of_data_bytes))
}

pub(crate) struct MessagePlanesIter<'a, T>
where
    T: Iterator<Item = u8>,
{
    pub(crate) message_byte_iter: &'a mut T,
    pub(crate) message_plane_length: usize,
    pub(crate) message_remnant_bit_number: usize,
}

impl<'a, T> MessagePlanesIter<'a, T>
where
    T: Iterator<Item = u8>,
{
    pub(crate) fn new(bytes_iter: &'a mut T) -> Self {
        MessagePlanesIter {
            message_byte_iter: bytes_iter,
            message_remnant_bit_number: 0,
            message_plane_length: 0,
        }
    }
}

impl<'a, T> Iterator for MessagePlanesIter<'a, T>
where
    T: Iterator<Item = u8>,
{
    type Item = BitPlane;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((plane_u8s, num_of_data_bytes)) = get_bytes_per_plane_u8s(self.message_byte_iter) {
            self.message_plane_length += 1;
            self.message_remnant_bit_number = num_of_data_bytes * 8;
            
            // get bits
            let mut data_bits: Vec<bool> = Vec::with_capacity(plane_u8s.len() * 8);
            for byte in plane_u8s {
                data_bits.extend(unsigned_int_to_bits(byte));
            }
            Some(BitPlane::from_bits(data_bits.try_into().unwrap()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_plane_iter_consistency() {
        let mut bytes = vec![
            0b00000000u8,
            0b00100000u8,
            0b11111111u8,
            0b10100101u8,
            0b00101011u8,
            0b11011001u8,
            0b11110111u8,
            0b00001101u8,
            0b00111101u8,
        ]
        .into_iter();

        let mut plane_iter = MessagePlanesIter::new(&mut bytes);

        let next = plane_iter.next().unwrap();
        assert_eq!(
            next.bits,
            [
                [false, false, false, false, false, false, false, false],
                [false, false, true, false, false, false, false, false],
                [true, true, true, true, true, true, true, true],
                [true, false, true, false, false, true, false, true],
                [false, false, true, false, true, false, true, true],
                [true, true, false, true, true, false, false, true],
                [true, true, true, true, false, true, true, true],
                [false, false, false, false, true, true, false, true],
            ]
        );

        let next = plane_iter.next().unwrap();
        assert_eq!(
            next.bits[0],
            [false, false, true, true, true, true, false, true]
        );

        let next = plane_iter.next();
        assert!(next.is_none());

        assert_eq!(plane_iter.message_plane_length, 2);
        assert_eq!(plane_iter.message_remnant_bit_number, 8);
    }
}
