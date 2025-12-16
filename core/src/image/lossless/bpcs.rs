//! All the functions to use BPCS (Bit Plane Complexity Segmentation) for embedding, extracting and estimating the
//! capacity of an image.
//!
//! # What is BPCS (Bit Plane Complexity Segmentation) Steganography?
//! BPCS steganography is a steganography method based on splitting an image into hundreds of thousands of small planes
//! and enumerating the value of each one to the total image.
//!
//! Here is a basic overview of the reoccurring function parameters:
//! * `min_alpha` — This parameter corresponds to the complexity threshold that is defined in the BPCS method. The value
//!   can range from 0.0 to 0.5 and it is a private component of the BPCS method. The higher the value, the less data
//!   you can embed, but the better the steganography quality.
//! * `rng_key` — This parameter is a parameter specific to this crate's implementation of BPCS, it controls the seed for
//!   the pseudo-random selection of which parts of the image to change. This parameter is also a private component.
//!
//! For an in depth review of the principles of BPCS please read [this paper](https://www.researchgate.net/file.PostFileLoader.html?id=53b3b80cd5a3f216068b4643&assetKey=AS%3A273551540588545%401442231177391).
//! As the link is old, it might not work anymore, if thats the case search for "Principle and Application of BPCS
//! Steganography" by E Kawaguchi.

pub(crate) mod bit_plane;
pub(crate) mod bit_plane_iter;
pub(crate) mod capacity;
pub(crate) mod dynamic_prefix;
pub(crate) mod initialization_vector;
pub(crate) mod message_plane_iter;
pub(crate) mod plane_selection;

use crate::{
    errors::SteganographyError,
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
    utils::image_utils::{image_to_binary_code, image_to_gray_code},
};
use image::RgbImage;
use itertools::Itertools;
use std::iter::zip;

/// Embed data into an image using BPCS
///
/// # Example
/// ```no_run
/// use pixelveil::{bpcs::embed_data, image_utils::open_rgbimage_from_path};
/// use image::RgbImage;
///
/// let mut vessel_image = RgbImage::new(512, 512);
/// let data: [u8; _] = [6, 9, 193, 7, 1, 7];
/// let min_alpha = 0.3f64;
/// let rng_key = [0u8; 32];
///
/// embed_data(
///     &mut vessel_image,
///     &mut data.into_iter(),
///     data.len(),
///     min_alpha,
///     rng_key,
/// ).unwrap();
/// ```
///
/// # Arguments
/// The function takes in five arguments:
/// * `source_image: &mut RgbImage` — A mutable reference to the source image.
/// * `data: &mut impl Iterator<Item = u8>` — An iterator that yields bytes (u8s), this is the data that is going to be
///   embedded. This was chosen to be an iterator to mitigate the memory usage of large amounts of data.
/// * `data_length: usize` — The length of the data iterator, in bytes (the number of u8s). Must be the exact length
///   of the `data` iterator.
/// * `min_alpha: f64` — The BPCS minimum complexity coefficient.
/// * `rng_key: [u8; 32]` — The randomization key, used for pseudo-random selection of where to change the source image.
///
/// # Errors
/// The errors that can be returned are:
/// * `SteganographyError::InsufficientPlaneNumber` — If the image doesn't contain enough bit planes to store the
/// inputted data.
///
/// # Returns
/// Returns `Result<(), Box<dyn std::error::Error>>`, the source image will be modified instead of returning a new one.
///
/// # Notes
/// For the best security, please use unique and original images and rng keys for each embedding operation as repeated
/// usage of these can lead to many attacks.
pub fn embed_data(
    source_image: &mut RgbImage,
    data: &mut impl Iterator<Item = u8>,
    data_length: usize,
    min_alpha: f64,
    rng_key: [u8; 32],
) -> Result<(), SteganographyError> {
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

/// Extract data from an image using BPCS
///
/// # Example
/// ```no_run
/// use pixelveil::{bpcs::extract_data, image_utils::open_rgbimage_from_path};
/// use image::RgbImage;
///
/// let mut vessel_image = RgbImage::new(512, 512);
/// let min_alpha = 0.3f64;
/// let rng_key = [0u8; 32];
///
/// let extracted_data = extract_data(
///     vessel_image,
///     min_alpha,
///     rng_key,
/// ).unwrap();
/// ```
///
/// # Arguments
/// The function takes in three arguments:
/// * `mut source_image: RgbImage` — The image to extract data from.
/// * `min_alpha: f64` — The BPCS minimum complexity coefficient.
/// * `rng_key: [u8; 32]` — The randomization key, used for pseudo-random selection of where to change the source image.
///
/// # Errors
/// The errors that can be returned are:
/// * `SteganographyError::InvalidIVData` — if the IV in the image contains invalid data. The most likely causes of this
/// are trying to extract data from an image that doesn't have data hidden in it or incorrect parameters.
///
/// # Returns
/// Returns `Result<i32, Box<Vec<u8> std::error::Error>>`. If `Ok(...)` is returned, the contained value is a vector of
/// the extracted data bytes.
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

/// Estimates the maximum payload capacity for an image that can be embedded using BPCS
///
/// The returned capacity already accounts for all internal overhead and represents the **actual available payload size
/// for the user**.
///
/// # Example
/// ```no_run
/// use pixelveil::bpcs::estimate_maximum_capacity;
/// use image::RgbImage;
///
/// let img = RgbImage::new(512, 512);
/// let capacity = estimate_maximum_capacity(&img, 0.3);
/// ```
///
/// # Arguments
/// The `estimate_maximum_capacity` function takes in:
/// * `source_image: &RgbImage` — The source image to analyze for BPCS embedding capacity.
/// * `min_alpha: f64` — The BPCS complexity threshold (0.0–0.5).  
///
/// # Errors
/// This function does not return errors.
///
/// # Returns
/// Returns a `u64` indicating the maximum number of payload bytes that can be embedded into `source_image` using BPCS,
/// after subtracting all internal overhead such as prefix data and IV planes.
///
/// # Notes
/// * A higher `min_alpha` typically reduces capacity because fewer bit-planes qualify as sufficiently complex.
/// * The result is deterministic for a given image and threshold.
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
