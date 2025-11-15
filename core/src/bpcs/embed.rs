use std::iter::zip;
use crate::{
    bpcs::{
        bit_plane::{get_planes_from_bits, get_planes_from_u8s, write_plane_at},
        capacity::check_capacity,
        initialization_vector::build_iv_planes,
        plane_selection::{AcceptedPlaneSelector, collect_accepted_planes},
    },
    utils::image_handling::{image_to_binary_code, image_to_gray_code},
};
use image::RgbImage;
use itertools::Itertools;

pub fn embed_data(
    source_image: &mut RgbImage,
    data: &[u8],
    min_alpha: f64,
    rng_key: [u8; 32],
) -> Result<(), Box<dyn std::error::Error>> {
    image_to_gray_code(source_image);

    let (mut message_planes, remnant_bit_number) = get_planes_from_u8s(data);
    let accepted_planes: Vec<(u32, u32, u8, u8)> = collect_accepted_planes(source_image, min_alpha)
        .into_iter()
        .collect_vec();
    let accepted_planes_num = accepted_planes.len();
    let mut plane_selector = AcceptedPlaneSelector::new(accepted_planes, rng_key);

    check_capacity(min_alpha, message_planes.len() as u32, accepted_planes_num)?;

    let mut conjugation_map: Vec<bool> = Vec::with_capacity(message_planes.len());
    for plane in &mut message_planes {
        if plane.alpha() < min_alpha {
            plane.conjugate();
            conjugation_map.push(true);
        } else {
            conjugation_map.push(false);
        }
    }

    let iv_plane_coords = plane_selector.select_iv_planes(min_alpha)?;
    let iv_planes = build_iv_planes(min_alpha, message_planes.len() as u32, remnant_bit_number);
    assert_eq!(iv_plane_coords.len(), iv_planes.len());
    let iv_pairs = zip(iv_plane_coords, iv_planes);

    let conj_map_plane_coords =
        plane_selector.select_conjugation_map_planes(min_alpha, message_planes.len() as u32)?;
    let (conj_map_planes, _) = get_planes_from_bits(conjugation_map);
    assert_eq!(conj_map_plane_coords.len(), conj_map_planes.len());
    let conj_map_pairs = zip(conj_map_plane_coords, conj_map_planes);

    let message_plane_coords = plane_selector.select_message_planes(message_planes.len() as u32)?;
    assert_eq!(message_plane_coords.len(), message_planes.len());
    let message_pairs = zip(message_plane_coords, message_planes);

    let embedding_pairs = iv_pairs.chain(conj_map_pairs).chain(message_pairs);

    for (coords, plane) in embedding_pairs {
        write_plane_at(source_image, plane, coords);
    }

    image_to_binary_code(source_image);

    Ok(())
}
