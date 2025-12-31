//! This crate provides steganography and steganalysis functionality in Rust as well as Python using PyO3.
//!
//! # Usage Considerations for all Steganography Operations
//! It is recommended to use original and unique vessels each time you embed data using any steganography method.
//! Please use original vessels that can't be found online. That way attackers won't be able to perform comparative
//! analysis of the original vessel and the one that has data hidden in it.
//! Please use different images when you perform multiple embedding operations, so attackers won't be able to compare
//! the different images.
//!
//! Additionally, some steganography methods support specific filetypes or compression methods. Each method will have
//! this info in its module level documentation.
//!
//! # Custom Structs and Types Used in this Project
//! The [RgbImage](https://docs.rs/image/latest/image/type.RgbImage.html) type is widely used in this project for
//! image processing in this crate. It represents a 24-bit RGB image, this specific type of image is the standard and
//! most common way to represent images.
//!
//! There are some functions that use the [DynamicImage](https://docs.rs/image/latest/image/enum.DynamicImage.html)
//! enum because they don't require a 24-bit RGB image specifically.
//!
//! For instructions and utility functions on how to open, handle, and export RgbImages and DynamicImages, consult the
//! [image_utils] module.
//!
//! # Steganography Functionality
//! Lossless Images:
//! * [BPCS (Bit Plane Complexity Segmentation)](bpcs)
//!
//! # Steganalysis Functionality
//! Image Steganalysis:
//! * [Subtract two images](image_steganalysis::subtract_images), [Subtract two pixels](image_steganalysis::subtract_pixels)
//! * [XOR two images](image_steganalysis::xor_images), [XOR two pixels](image_steganalysis::xor_pixels)
//! * [Highlight every different channel in every pixel between two images](image_steganalysis::highlight_image_difference)
//! * [Slice an image into 24 bit planes](image_steganalysis::slice_image_bit_planes)
//!
//! # Future Development
//! In the future I plan on implementing the following steganography methods (not in this particular order):
//! * Image LSB (Least Significant Bit) — only for lossless filetypes
//! * Lossless audio filetype (.wav) method(s)
//! * Lossy audio filetype (.mp3) method(s)
//! * Video steganography methods (.mp4)
//!
//! Because I am a One Man Operation™, I need time to research, prototype, and finalize each one of these
//! steganography methods. So don't hold your breath :)

#![warn(missing_docs)]

#[macro_use]
extern crate itertools;

pub mod errors;
pub mod image;
pub mod steganalysis;
pub mod utils;

// re-exports
pub use image::lossless::bpcs;
pub use steganalysis::image_steganalysis;
pub use utils::image_utils;
