use std::{
    error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum SteganographyError {
    InsufficientPlaneNumber(usize, usize), // the number that was expected of the operation, the number there are
    InvalidIVData(String),                 // the explanation
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
                write!(
                    f,
                    "The extracted IV data is invalid: {reason}. This most probable cause are incorrect running parameters."
                )
            }
        }
    }
}

impl error::Error for SteganographyError {}

pub(crate) fn check_plane_number(expected: usize, got: usize) -> Result<(), SteganographyError> {
    if expected > got {
        Err(SteganographyError::InsufficientPlaneNumber(expected, got))
    } else {
        Ok(())
    }
}
