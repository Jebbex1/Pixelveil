use crate::{
    errors::SteganographyError,
    image::lossless::bpcs::{
        bit_plane_iter::BitIndexedBitPlaneIter,
        dynamic_prefix::{num_of_prefixed_planes_for_n_bits, prefix_length},
        initialization_vector::{MESSAGE_LENGTH_IV_BIT_NUMBER, MESSAGE_REMNANT_IV_BIT_NUMBER},
    },
};
use image::RgbImage;
use itertools::Itertools;
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
use std::collections::HashMap;

pub(crate) fn count_accepted_planes(source_image: &RgbImage, min_alpha: f64) -> u64 {
    let mut accepted_count = 0u64;
    for bit_index in 0..8 {
        let plane_iter = BitIndexedBitPlaneIter::new(source_image, bit_index);
        for (_, plane) in plane_iter {
            if plane.alpha() >= min_alpha {
                accepted_count += 1;
            }
        }
    }
    accepted_count
}

pub(crate) fn collect_accepted_planes_at_bit_index(
    source_image: &RgbImage,
    min_alpha: f64,
    bit_index: u8,
) -> Vec<(u32, u32, u8, u8)> {
    let mut accepted_coords: Vec<(u32, u32, u8, u8)> = Vec::new();

    let plane_iter = BitIndexedBitPlaneIter::new(source_image, bit_index);
    for ((x, y, channel, bit_index), plane) in plane_iter {
        if plane.alpha() >= min_alpha {
            accepted_coords.push((x, y, channel, bit_index));
        }
    }

    accepted_coords
}

// TODO: make this generic
pub(crate) fn select_n_planes_from_vec(
    source_planes: &mut Vec<(u32, u32, u8, u8)>,
    plane_number: usize,
    rng: &mut StdRng,
) -> Result<Vec<(u32, u32, u8, u8)>, SteganographyError> {
    assert!(plane_number <= source_planes.len());
    let mut selected_planes: Vec<(u32, u32, u8, u8)> = Vec::with_capacity(plane_number);
    let mut indexes = (0..source_planes.len()).collect_vec();

    indexes.shuffle(rng);
    let mut selected_indexes = indexes[0..plane_number].to_vec();

    selected_indexes.sort();
    selected_indexes.reverse();

    for i in selected_indexes {
        let p = source_planes.swap_remove(i);
        selected_planes.push(p);
    }

    selected_planes.shuffle(rng);

    Ok(selected_planes)
}

pub(crate) struct PlaneSelector<'a> {
    source_image: &'a RgbImage,
    min_alpha: f64,
    plane_map: HashMap<u8, Option<Vec<(u32, u32, u8, u8)>>>,
    rng: StdRng,
}

impl<'a> PlaneSelector<'a> {
    pub(crate) fn new(
        source_image: &'a RgbImage,
        min_alpha: f64,
        randomization_seed: [u8; 32],
    ) -> Self {
        // Generate empty map
        let mut plane_map: HashMap<u8, Option<Vec<(u32, u32, u8, u8)>>> = HashMap::with_capacity(8);
        for bit_index in 0u8..8u8 {
            plane_map.insert(bit_index, None);
        }

        PlaneSelector {
            source_image,
            min_alpha,
            plane_map,
            rng: StdRng::from_seed(randomization_seed),
        }
    }

    pub(crate) fn select_n_planes(
        &mut self,
        n: usize,
    ) -> Result<Vec<(u32, u32, u8, u8)>, SteganographyError> {
        let mut unselected_num = n;
        let mut total_selected: Vec<(u32, u32, u8, u8)> = Vec::with_capacity(n);

        for bit_index in (0u8..8u8).rev() {
            // if the current bit index accepted planes weren't mapped yet, get them
            match self.plane_map.get(&bit_index).unwrap() {
                None => {
                    self.plane_map.insert(
                        bit_index,
                        Some(collect_accepted_planes_at_bit_index(
                            self.source_image,
                            self.min_alpha,
                            bit_index,
                        )),
                    );
                }
                _ => {}
            }

            let curr_bit_index_planes = self
                .plane_map
                .get_mut(&bit_index)
                .unwrap()
                .as_mut()
                .unwrap();

            if unselected_num < curr_bit_index_planes.len() {
                let curr_bit_index_selected =
                    select_n_planes_from_vec(curr_bit_index_planes, unselected_num, &mut self.rng)?;

                unselected_num -= curr_bit_index_selected.len();

                total_selected.extend(curr_bit_index_selected);
                break;
            } else {
                unselected_num -= curr_bit_index_planes.len();

                total_selected.extend(curr_bit_index_planes.drain(..));

                continue;
            }
        }

        if unselected_num != 0 {
            return Err(SteganographyError::InsufficientPlaneNumber(
                n,
                total_selected.len(),
            ));
        }

        total_selected.shuffle(&mut self.rng);
        Ok(total_selected)
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
    use image::open;

    use super::*;

    #[test]
    fn test_deterministic_plane_selection() -> Result<(), Box<dyn std::error::Error>> {
        let min_alpha: f64 = 0.2f64;
        let randomization_seed = [0u8; 32];
        let message_plane_length = 15_000usize;
        let image_path = "tests/assets/test_deterministic_plane_selection.png";
        let source_image = open(image_path)?.to_rgb8();

        let mut selector1 = PlaneSelector::new(&source_image, min_alpha, randomization_seed);

        let iv_planes1 = selector1.select_iv_planes(min_alpha)?;
        let conj_map_planes1 =
            selector1.select_conjugation_map_planes(min_alpha, message_plane_length)?;
        let message_planes1 = selector1.select_message_planes(message_plane_length)?;

        let mut selector2 = PlaneSelector::new(&source_image, min_alpha, randomization_seed);

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
        let randomization_seed = [0u8; 32];
        let image_path = "tests/assets/test_failing_plane_selection.png";
        let source_image = open(image_path)?.to_rgb8();

        let mut selector = PlaneSelector::new(&source_image, min_alpha, randomization_seed);

        selector.select_iv_planes(min_alpha)?;
        selector.select_conjugation_map_planes(min_alpha, message_plane_length)?;

        // On this line the selector should have insufficient unselected planes.
        let result = selector.select_message_planes(message_plane_length);
        assert!(result.is_err());

        Ok(())
    }
}
