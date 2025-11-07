use crate::bpcs::bit_plane_iter::BitPlaneIter;
use image::RgbImage;
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

pub(crate) fn collect_accepted_blocks(
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
