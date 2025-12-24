//! All of the custom errors that can be returned in a Result value in this package's functions

use std::{
    error,
    fmt::{self, Display, Formatter},
};

/// All steganography-related errors
#[derive(Debug)]
pub enum SteganographyError {
    /// Occurs when an embedded IV stores invalid data
    ///
    /// This error will be propagated when the IV contains data that is impossible (e.g. if an IV claims that some
    /// value is beyond it's known constraints).
    ///
    /// The stored value represents the explanation to why the data that the IV contains is invalid.
    InvalidIVData(String),

    /// Occurs when an image doesn't have the minimum amount of accepted bit planes to perform an operation.
    ///
    /// This is a BPCS specific error and can happen while embedding and extracting.
    ///
    /// The stored values are (in this order):
    /// * The minimum number of accepted and unused bit planes that the image was expected to contain
    /// * The number of accepted and unused bit planes that was found
    InsufficientPlaneNumber(usize, usize),
}

impl Display for SteganographyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::InsufficientPlaneNumber(expected, got) => {
                write!(
                    f,
                    "Tried to do an operation that requires at least {expected} planes, got {got}"
                )
            }
            Self::InvalidIVData(reason) => {
                write!(f, "The extracted IV data is invalid: {reason}")
            }
        }
    }
}
impl error::Error for SteganographyError {}
