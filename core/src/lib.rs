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
//! # Supported Steganography Methods
//! Lossless Images:
//! * [BPCS (Bit Plane Complexity Segmentation)](bpcs)
//!
//! # Steganalysis Functionality
//!
//!
//! # Future Development
//! In the future I plan on implementing the following steganography methods (Not in this particular order):
//! * Image LSB (Least Significant Bit) — only for lossless filetypes
//! * Lossless audio filetype (.wav) method(s)
//! * Lossy audio filetype (.mp3) method(s)
//! * Video steganography methods (.mp4)
//!
//! Because I am a One Man Operation™, I need to take time to research, prototype, and finalize each one of these
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
