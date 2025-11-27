use crate::{
    image::lossless::bpcs::bit_plane::{BYTES_PER_PLANE, BitPlane},
    utils::bit_operations::unsigned_int_to_bits,
};
use rand::{Rng, rng};

pub(crate) fn get_bytes_per_plane_u8s<'a, T>(iter: &'a mut T) -> Option<Vec<u8>>
where
    T: Iterator<Item = u8>,
{
    let mut u8s: Vec<u8> = Vec::with_capacity(BYTES_PER_PLANE);

    for _ in 0..BYTES_PER_PLANE {
        // if the next item is found, push it.
        // if not, check if the u8s vec contains any data, if it does, fill it with random data
        // if not, return None
        if let Some(next) = iter.next() {
            u8s.push(next);
        } else {
            if u8s.is_empty() {
                return None;
            } else {
                u8s.push(rng().random_range(0u8..0b11111111u8));
            }
        }
    }

    Some(u8s)
}

pub(crate) struct MessagePlanesIter<'a, T>
where
    T: Iterator<Item = u8>,
{
    pub(crate) message_byte_iter: &'a mut T,
    pub(crate) conjugation_map: &'a mut Vec<bool>,
}

impl<'a, T> MessagePlanesIter<'a, T>
where
    T: Iterator<Item = u8>,
{
    pub(crate) fn new(message_byte_iter: &'a mut T, conjugation_map: &'a mut Vec<bool>) -> Self {
        MessagePlanesIter {
            message_byte_iter,
            conjugation_map,
        }
    }
}

impl<'a, T> Iterator for MessagePlanesIter<'a, T>
where
    T: Iterator<Item = u8>,
{
    type Item = BitPlane;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(plane_u8s) = get_bytes_per_plane_u8s(self.message_byte_iter) {
            // get bits
            let mut data_bits: Vec<bool> = Vec::with_capacity(plane_u8s.len() * 8);
            for byte in plane_u8s {
                data_bits.extend(unsigned_int_to_bits(byte));
            }

            let mut plane = BitPlane::from_bits(data_bits.try_into().unwrap());

            if plane.alpha() < 0.5 {
                plane.conjugate();
                self.conjugation_map.push(true);
            } else {
                self.conjugation_map.push(false);
            }

            Some(plane)
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
        let mut conj_map: Vec<bool> = Vec::with_capacity(8 * 9);

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

        let mut plane_iter = MessagePlanesIter::new(&mut bytes, &mut conj_map);

        let mut next = plane_iter.next().unwrap();
        next.conjugate(); // next is supposed to be automatically conjugated on the .next(), so we conjugate it again to get the original data
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
        // the 2nd bit plane might have been conjugated, depending on how the bits were filled to fit a plane, so we need the
        //  first row (the data bits) to be equal to either the regular or conjugated version of itself
        assert!(
            (next.bits[0] == [false, false, true, true, true, true, false, true])
                ^ (next.bits[0] == [false, true, true, false, true, false, false, false])
        );

        let next = plane_iter.next();
        assert!(next.is_none());

        assert_eq!(conj_map[0], true);
    }
}
