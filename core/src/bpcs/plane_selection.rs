use crate::{
    bpcs::{
        bit_plane_iter::BitPlaneIter,
        dynamic_prefix::{num_of_prefixed_planes_for_n_bits, prefix_length},
        initialization_vector::{MESSAGE_LENGTH_IV_BIT_NUMBER, MESSAGE_REMNANT_IV_BIT_NUMBER},
    },
    errors::{SteganographyError, check_plane_number},
};
use image::RgbImage;
use rand::{
    SeedableRng,
    rngs::StdRng,
    seq::{IteratorRandom, SliceRandom},
};
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

pub(crate) struct AcceptedPlaneSelector {
    accepted_planes: Vec<(u32, u32, u8, u8)>,
    rng: StdRng,
}

impl AcceptedPlaneSelector {
    fn new(accepted_planes: Vec<(u32, u32, u8, u8)>, randomization_seed: [u8; 32]) -> Self {
        AcceptedPlaneSelector {
            accepted_planes,
            rng: StdRng::from_seed(randomization_seed),
        }
    }

    // will be used for selection of planes for both iv and conj map planes
    fn select_small_n_planes(
        &mut self,
        plane_number: usize,
    ) -> Result<Vec<(u32, u32, u8, u8)>, SteganographyError> {
        check_plane_number(plane_number, self.accepted_planes.len())?;
        let mut selected_planes: Vec<(u32, u32, u8, u8)> = Vec::with_capacity(plane_number);
        let selected_indexes =
            (0..self.accepted_planes.len()).choose_multiple(&mut self.rng, plane_number);

        for i in selected_indexes {
            let p = self.accepted_planes.swap_remove(i);
            selected_planes.push(p);
        }

        Ok(selected_planes)
    }

    // will be used to select message planes only - so it is used once and only at the end
    fn select_big_n_planes(
        mut self,
        plane_number: usize,
    ) -> Result<Vec<(u32, u32, u8, u8)>, SteganographyError> {
        check_plane_number(plane_number, self.accepted_planes.len())?;
        self.accepted_planes.shuffle(&mut self.rng);
        Ok(self.accepted_planes[0..plane_number].to_vec())
    }

    fn select_iv_planes(
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
        self.select_small_n_planes(iv_plane_num)
    }

    pub(crate) fn select_conjugation_map_planes(
        &mut self,
        min_alpha: f64,
        message_plane_length: u32,
    ) -> Result<Vec<(u32, u32, u8, u8)>, SteganographyError> {
        let conjugation_map_plane_num = num_of_prefixed_planes_for_n_bits(
            message_plane_length as usize,
            prefix_length(min_alpha),
        );
        self.select_small_n_planes(conjugation_map_plane_num)
    }

    pub(crate) fn select_message_planes(
        self,
        message_plane_length: u32,
    ) -> Result<Vec<(u32, u32, u8, u8)>, SteganographyError> {
        self.select_big_n_planes(message_plane_length as usize)
    }
}

#[cfg(test)]
mod tests {
    use image::open;

    use super::*;

    #[test]
    fn test_deterministic_plane_selection() -> Result<(), Box<dyn std::error::Error>> {
        let min_alpha = 0.2f64;
        let message_plane_length = 82u32;
        let accepted_planes = collect_accepted_planes(
            &open("tests/assets/test_deterministic_plane_selection.png")?.to_rgb8(),
            min_alpha,
        );
        let randomization_seed = [0u8; 32];

        let mut selector1 = AcceptedPlaneSelector::new(
            accepted_planes.clone().into_iter().collect::<Vec<_>>(),
            randomization_seed,
        );

        let iv_planes1 = selector1.select_iv_planes(min_alpha)?;
        let conj_map_planes1 =
            selector1.select_conjugation_map_planes(min_alpha, message_plane_length)?;
        let message_planes1 = selector1.select_message_planes(message_plane_length)?;

        let mut selector2 = AcceptedPlaneSelector::new(
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
        let message_plane_length = 40u32;
        let accepted_planes = collect_accepted_planes(
            &open("tests/assets/test_failing_plane_selection.png")?.to_rgb8(),
            min_alpha,
        );
        let randomization_seed = [0u8; 32];

        let mut selector = AcceptedPlaneSelector::new(
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
