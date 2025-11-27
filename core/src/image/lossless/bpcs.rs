pub(crate) mod bit_plane;
pub(crate) mod bit_plane_iter;
pub(crate) mod capacity;
pub(crate) mod dynamic_prefix;
pub(crate) mod initialization_vector;
pub(crate) mod message_plane_iter;
pub(crate) mod plane_selection;

use crate::{
    image::lossless::bpcs::{
        bit_plane::{
            BYTES_PER_PLANE, USIZE_PLANE_SIZE, get_planes_from_image_and_coords, write_plane_at,
        },
        dynamic_prefix::{num_of_prefixed_planes_for_n_bits, prefix_length},
        initialization_vector::{
            MESSAGE_LENGTH_IV_BIT_NUMBER, MESSAGE_REMNANT_IV_BIT_NUMBER,
            build_conjugation_map_planes, build_iv_planes,
            extract_conj_map_data_from_conj_map_planes, extract_iv_data_from_iv_planes,
        },
        message_plane_iter::MessagePlanesIter,
        plane_selection::{PlaneSelector, count_accepted_planes},
    },
    utils::image_handling::{image_to_binary_code, image_to_gray_code},
};
use image::RgbImage;
use itertools::Itertools;
use std::iter::zip;

pub fn embed_data(
    source_image: &mut RgbImage,
    data: &mut impl Iterator<Item = u8>,
    data_length: usize,
    min_alpha: f64,
    rng_key: [u8; 32],
) -> Result<(), Box<dyn std::error::Error>> {
    image_to_gray_code(source_image);

    // calculate all the necessary values for the initialization vectors and such
    let message_plane_length = (data_length as f64 / BYTES_PER_PLANE as f64).ceil() as usize;
    let mut remnant_bit_number = (data_length * 8) % (USIZE_PLANE_SIZE * USIZE_PLANE_SIZE);
    remnant_bit_number = if remnant_bit_number == 0 {
        USIZE_PLANE_SIZE * USIZE_PLANE_SIZE
    } else {
        remnant_bit_number
    };

    // crate conjugation map
    let mut conjugation_map: Vec<bool> = Vec::with_capacity(message_plane_length);

    // patch the data iterator and conjugation map into the message plane iter
    let message_plane_iter = MessagePlanesIter::new(data, &mut conjugation_map);

    // collect the accepted planes and put them in a PRNG selector
    let mut plane_selector = PlaneSelector::new(&source_image, min_alpha, rng_key);

    // select all planes
    let iv_plane_coords = plane_selector.select_iv_planes(min_alpha)?;
    let conj_map_plane_coords =
        plane_selector.select_conjugation_map_planes(min_alpha, message_plane_length)?;
    let message_plane_coords = plane_selector.select_message_planes(message_plane_length)?;

    // embed IV
    let iv_planes = build_iv_planes(min_alpha, message_plane_length, remnant_bit_number);
    assert_eq!(iv_plane_coords.len(), iv_planes.len());
    let iv_pairs = zip(iv_plane_coords, iv_planes);

    for (coords, plane) in iv_pairs {
        write_plane_at(source_image, plane, coords);
    }

    // embed message (and by that we construct the conjugation map)
    assert_eq!(message_plane_coords.len(), message_plane_length);
    let message_pairs = zip(message_plane_coords, message_plane_iter);
    for (coords, plane) in message_pairs {
        write_plane_at(source_image, plane, coords);
    }

    // embed conjugation map
    let conj_map_planes = build_conjugation_map_planes(conjugation_map, min_alpha);

    assert_eq!(conj_map_plane_coords.len(), conj_map_planes.len());
    let conj_map_pairs = zip(conj_map_plane_coords, conj_map_planes);
    for (coords, plane) in conj_map_pairs {
        write_plane_at(source_image, plane, coords);
    }

    image_to_binary_code(source_image);

    Ok(())
}

pub fn extract_data(
    mut source_image: RgbImage,
    min_alpha: f64,
    rng_key: [u8; 32],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    image_to_gray_code(&mut source_image);

    let mut selector = PlaneSelector::new(&source_image, min_alpha, rng_key);

    let iv_planes =
        get_planes_from_image_and_coords(&source_image, selector.select_iv_planes(min_alpha)?);

    let (message_plane_length, message_remnant_length) =
        extract_iv_data_from_iv_planes(iv_planes, min_alpha)?;

    let conjugation_map_planes = get_planes_from_image_and_coords(
        &source_image,
        selector.select_conjugation_map_planes(min_alpha, message_plane_length)?,
    );

    let conjugation_map = extract_conj_map_data_from_conj_map_planes(
        conjugation_map_planes,
        min_alpha,
        message_plane_length,
    )?;

    let message_planes = get_planes_from_image_and_coords(
        &source_image,
        selector.select_message_planes(message_plane_length)?,
    );

    assert_eq!(conjugation_map.len(), message_planes.len());

    let mut data: Vec<u8> = Vec::with_capacity(message_plane_length * BYTES_PER_PLANE);

    for (is_conjugated, mut plane) in zip(conjugation_map, message_planes) {
        if is_conjugated {
            plane.conjugate();
        }
        data.extend(plane.export_to_u8s());
    }

    let final_length = ((message_plane_length - 1) * BYTES_PER_PLANE) + message_remnant_length / 8;

    Ok(data.drain(0..final_length).collect_vec())
}

pub fn estimate_maximum_capacity(source_image: &RgbImage, min_alpha: f64) -> u64 {
    let accepted_plane_number = count_accepted_planes(source_image, min_alpha);
    let prefix_length = prefix_length(min_alpha);
    let iv_planes_num =
        (num_of_prefixed_planes_for_n_bits(MESSAGE_LENGTH_IV_BIT_NUMBER, prefix_length)
            + num_of_prefixed_planes_for_n_bits(MESSAGE_REMNANT_IV_BIT_NUMBER, prefix_length))
            as u64;

    ((accepted_plane_number - 2 - iv_planes_num) as f64
        / (1.0 + (1 / ((USIZE_PLANE_SIZE * USIZE_PLANE_SIZE) - prefix_length)) as f64))
        .floor() as u64
        * (BYTES_PER_PLANE as u64)
}
