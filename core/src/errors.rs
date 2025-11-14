use std::{
    error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub(crate) enum SteganographyError {
    InsufficientPlaneNumber(usize, usize), // the number that was expected of the operation, the number there are
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
