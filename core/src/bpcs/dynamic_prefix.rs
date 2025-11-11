use crate::bpcs::bit_plane::{BitPlane, PLANE_SIZE, USIZE_PLANE_SIZE};
use rand::Rng;

pub(crate) fn num_of_prefixed_planes_for_n_bits(n: usize, prefix_length: usize) -> usize {
    (n as f64 / ((USIZE_PLANE_SIZE * USIZE_PLANE_SIZE) - prefix_length) as f64).ceil() as usize
}

pub(crate) fn prefix_length(min_alpha: f64) -> usize {
    ((PLANE_SIZE * PLANE_SIZE) as f64 * ((1.4 * min_alpha) + 0.05)).ceil() as usize
}

pub(crate) fn get_n_random_bools(n: usize) -> Vec<bool> {
    let mut rng = rand::rng();
    let mut bools = Vec::with_capacity(n);
    for _ in 0..n {
        bools.push(rng.random_bool(0.5));
    }
    bools
}

pub(crate) fn fill_to_plane_size(bits: &mut Vec<bool>) {
    let remainder = bits.len() % (USIZE_PLANE_SIZE * USIZE_PLANE_SIZE);
    if remainder == 0 {
        return;
    }
    let filler_bits = get_n_random_bools(remainder);
    bits.extend(filler_bits.into_iter());
}

pub(crate) fn get_next_prefixed_plane(
    bits: &mut Vec<bool>,
    min_alpha: f64,
    prefix_length: usize,
) -> BitPlane {
    assert!(
        bits.len() >= (USIZE_PLANE_SIZE * USIZE_PLANE_SIZE) - prefix_length,
        "Tried to construct block with an insufficient amount of bits to fill up a prefixed block."
    );
    let plane_data: Vec<bool> = bits
        .drain(0..((USIZE_PLANE_SIZE * USIZE_PLANE_SIZE) - prefix_length))
        .collect();
    loop {
        let prefix_bits = get_n_random_bools(prefix_length);
        let plane_bits = [prefix_bits.as_slice(), plane_data.as_slice()].concat();
        let plane = BitPlane::from_bits(plane_bits.try_into().unwrap());
        if plane.alpha() >= min_alpha {
            return plane;
        } // if the constructed block is acceptable, return it. else: construct a new one.
    }
}

pub(crate) fn get_prefixed_planes(mut bits: Vec<bool>, min_alpha: f64) -> Vec<BitPlane> {
    let prefix_length = prefix_length(min_alpha);
    let mut planes: Vec<BitPlane> = Vec::new();
    while !bits.is_empty() {
        planes.push(get_next_prefixed_plane(&mut bits, min_alpha, prefix_length));
    }
    planes
}

pub(crate) fn data_bits_from_prefixed_planes(planes: Vec<BitPlane>, min_alpha: f64) -> Vec<bool> {
    let prefix_length = prefix_length(min_alpha);
    let mut data: Vec<bool> = Vec::new();
    for plane in planes {
        let mut plane_bits = plane.export_to_bools().to_vec();
        data.extend(plane_bits.drain(prefix_length..));
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_prefixed_plane() {
        let mut bits = vec![false; 40];
        let next_plane = get_next_prefixed_plane(&mut bits, 0.3, prefix_length(0.3));
        assert!(next_plane.alpha() >= 0.3);
        assert_eq!(bits, [false; 7]);
    }

    #[test]
    fn test_circular_plane_prefixing_and_data_extraction() {
        let min_alpha: f64 = 0.3;
        let bits = get_n_random_bools(99);
        let planes = get_prefixed_planes(bits.clone(), min_alpha);
        let data = data_bits_from_prefixed_planes(planes, min_alpha);
        assert_eq!(data, bits);
    }

    #[test]
    fn test_num_of_prefixed_planes_for_n_bits() {
        assert_eq!(num_of_prefixed_planes_for_n_bits(128, 0), 2);
        assert_eq!(num_of_prefixed_planes_for_n_bits(40, 24), 1);
        assert_eq!(num_of_prefixed_planes_for_n_bits(79, 31), 3);
    }
}
