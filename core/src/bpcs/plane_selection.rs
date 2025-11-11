use crate::bpcs::bit_plane_iter::BitPlaneIter;
use image::RgbImage;
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
use std::collections::HashSet;

pub(crate) fn count_accepted_planes(source_image: &RgbImage, min_alpha: f64) -> u64 {
    let plane_iter = BitPlaneIter::new(source_image);
    let mut count: u64 = 0;
    for (_, plane) in plane_iter {
        if plane.alpha() >= min_alpha {
            count += 1;
        }
    }
    count
}

pub(crate) fn collect_accepted_planes(
    source_image: &RgbImage,
    min_alpha: f64,
) -> HashSet<(u32, u32, u8, u8)> {
    let plane_iter = BitPlaneIter::new(source_image);
    let mut accepted_coords: HashSet<(u32, u32, u8, u8)> = HashSet::new();
    for (coords, plane) in plane_iter {
        if plane.alpha() >= min_alpha {
            accepted_coords.insert(coords);
        }
    }
    accepted_coords
}

pub(crate) fn select_n_random_planes(
    coords_vec: &mut Vec<(u32, u32, u8, u8)>,
    rng_seed: [u8; 32],
    n: usize,
) -> Vec<(u32, u32, u8, u8)> {
    let mut rng = StdRng::from_seed(rng_seed);
    coords_vec.shuffle(&mut rng);
    coords_vec[0..n].to_vec()
}

pub(crate) fn select_n_accepted_planes(
    source_image: &RgbImage,
    min_alpha: f64,
    rng_seed: [u8; 32],
    n: usize,
) -> Vec<(u32, u32, u8, u8)> {
    let coords_set = collect_accepted_planes(source_image, min_alpha);
    let mut coords_vec: Vec<(u32, u32, u8, u8)> = coords_set.into_iter().collect();
    select_n_random_planes(&mut coords_vec, rng_seed, n)
}
