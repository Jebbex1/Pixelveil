use crate::{
    errors::{SteganographyError, check_plane_number},
    image::lossless::bpcs::{
        bit_plane::PLANE_SIZE,
        bit_plane_iter::BitPlaneIter,
        dynamic_prefix::{num_of_prefixed_planes_for_n_bits, prefix_length},
        initialization_vector::{MESSAGE_LENGTH_IV_BIT_NUMBER, MESSAGE_REMNANT_IV_BIT_NUMBER},
    },
};
use image::RgbImage;
use rand::{
    SeedableRng,
    rngs::StdRng,
    seq::{IteratorRandom, SliceRandom},
};
use rand_distr::{Distribution, weighted::WeightedIndex};
use statrs::distribution::{Continuous, Normal};

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
) -> Vec<(u32, u32, u8, u8)> {
    let plane_iter = BitPlaneIter::new(source_image);
    let mut accepted_coords: Vec<(u32, u32, u8, u8)> = Vec::new();
    for (coords, plane) in plane_iter {
        if plane.alpha() >= min_alpha {
            accepted_coords.push((
                coords.0 * PLANE_SIZE,
                coords.1 * PLANE_SIZE,
                coords.2,
                coords.3,
            ));
        }
    }
    accepted_coords
}

const NORMAL_DIST_MEAN: f64 = 9.0;
const NORMAL_DIST_STD_DEV: f64 = 2.6;

pub(crate) struct WeightedPlaneSelector {
    accepted_planes: Vec<(u32, u32, u8, u8)>,
    weights: Vec<f64>,
    rng: StdRng,
}

impl WeightedPlaneSelector {
    pub(crate) fn new(
        accepted_planes: Vec<(u32, u32, u8, u8)>,
        randomization_seed: [u8; 32],
    ) -> Self {
        let dist = Normal::new(NORMAL_DIST_MEAN, NORMAL_DIST_STD_DEV).unwrap();

        let pdfs: Vec<f64> = accepted_planes.iter().map(|&(_, _, _, bit_index)| dist.pdf(1.0 + bit_index as f64)).collect();

        WeightedPlaneSelector {
            accepted_planes,
            weights: pdfs,
            rng: StdRng::from_seed(randomization_seed),
        }
    }

    pub(crate) fn select_n_planes(&mut self, n: usize) -> Result<Vec<(u32, u32, u8, u8)>, SteganographyError> {
        if n > self.accepted_planes.len() {
            return Err(SteganographyError::InsufficientPlaneNumber(n, self.accepted_planes.len()))
        }

        let mut selected: Vec<(u32, u32, u8, u8)> = Vec::with_capacity(n);

        for _ in 0..n {
            let dist = WeightedIndex::new(&*self.weights).unwrap();
            let selected_index = dist.sample(&mut self.rng);

            selected.push(self.accepted_planes.swap_remove(selected_index));
            self.weights.swap_remove(selected_index);
        }

        Ok(selected)
    }

    pub(crate) fn select_iv_planes(
        &mut self,
        min_alpha: f64,
    ) -> Result<Vec<(u32, u32, u8, u8)>, SteganographyError> {
        let iv_plane_num = num_of_prefixed_planes_for_n_bits(
            MESSAGE_LENGTH_IV_BIT_NUMBER,
            prefix_length(min_alpha),
        ) + num_of_prefixed_planes_for_n_bits(
            MESSAGE_REMNANT_IV_BIT_NUMBER,
            prefix_length(min_alpha),
        );
        self.select_n_planes(iv_plane_num)
    }

    pub(crate) fn select_conjugation_map_planes(
        &mut self,
        min_alpha: f64,
        message_plane_length: usize,
    ) -> Result<Vec<(u32, u32, u8, u8)>, SteganographyError> {
        let conjugation_map_plane_num =
            num_of_prefixed_planes_for_n_bits(message_plane_length, prefix_length(min_alpha));
        self.select_n_planes(conjugation_map_plane_num)
    }

    pub(crate) fn select_message_planes(
        &mut self,
        message_plane_length: usize,
    ) -> Result<Vec<(u32, u32, u8, u8)>, SteganographyError> {
        self.select_n_planes(message_plane_length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::open;

    #[test]
    fn test_deterministic_plane_selection() -> Result<(), Box<dyn std::error::Error>> {
        let min_alpha = 0.2f64;
        let message_plane_length = 82usize;
        let accepted_planes = collect_accepted_planes(
            &open("tests/assets/test_deterministic_plane_selection.png")?.to_rgb8(),
            min_alpha,
        );
        let randomization_seed = [0u8; 32];

        let mut selector1 = WeightedPlaneSelector::new(
            accepted_planes.clone().into_iter().collect::<Vec<_>>(),
            randomization_seed,
        );

        let iv_planes1 = selector1.select_iv_planes(min_alpha)?;
        let conj_map_planes1 =
            selector1.select_conjugation_map_planes(min_alpha, message_plane_length)?;
        let message_planes1 = selector1.select_message_planes(message_plane_length)?;

        let mut selector2 = WeightedPlaneSelector::new(
            accepted_planes.clone().into_iter().collect::<Vec<_>>(),
            randomization_seed,
        );

        let iv_planes2 = selector2.select_iv_planes(min_alpha)?;
        let conj_map_planes2 =
            selector2.select_conjugation_map_planes(min_alpha, message_plane_length)?;
        let message_planes2 = selector2.select_message_planes(message_plane_length)?;

        assert_eq!(iv_planes1, iv_planes2);
        assert_eq!(conj_map_planes1, conj_map_planes2);
        assert_eq!(message_planes1, message_planes2);

        Ok(())
    }

    #[test]
    fn test_failing_plane_selection() -> Result<(), Box<dyn std::error::Error>> {
        let min_alpha = 0.2f64;
        let message_plane_length = 40usize;
        let accepted_planes = collect_accepted_planes(
            &open("tests/assets/test_failing_plane_selection.png")?.to_rgb8(),
            min_alpha,
        );
        let randomization_seed = [0u8; 32];

        let mut selector = WeightedPlaneSelector::new(
            accepted_planes.into_iter().collect::<Vec<_>>(),
            randomization_seed,
        );
        selector.select_iv_planes(min_alpha)?;
        selector.select_conjugation_map_planes(min_alpha, message_plane_length)?;

        // On this line the selector should have insufficient unselected planes.
        let result = selector.select_message_planes(message_plane_length);
        assert!(result.is_err());

        Ok(())
    }
}
