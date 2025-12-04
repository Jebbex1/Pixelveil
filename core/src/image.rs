//! Functionality regarding lossy and lossless image types
//!
//! This crate uses the [image](https://crates.io/crates/image) and specifically the [RgbImage](https://docs.rs/image/0.25.9/image/type.RgbImage.html)
//! type alias to represent 24-bit RGB images.
//!
//! # How to Open `RgbImage`s
//! Opening from a path using the image crate:
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let your_path = "assets/image.png";
//!     let image = image::open(your_path)?.to_rgb8();
//! #     Ok(())
//! # }
//! ```
//!
//! Opening from a path using this crates utility function:
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #     use pixelveil::image_utils::open_rgbimage_from_path;
//!     let your_path = "assets/image.png";
//!     let image = open_rgbimage_from_path(your_path)?;
//! #     Ok(())
//! # }
//! ```
//!
//! To not force developers to read images from in their file system, you can also read from raw data.
//!
//! Opening from raw data:
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #     use pixelveil::image_utils::open_rgbimage_from_raw;
//!     let your_path = "assets/image.png";
//!     let raw_data: Vec<u8> = std::fs::read(your_path)?; // this is an example, but a file might be sent over a connection or another medium
//!     let image = open_rgbimage_from_raw(raw_data)?;
//! #     Ok(())
//! # }
//! ```
//!

pub mod lossless;
