use std::{
    error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub(crate) enum SteganographyError {
    InsufficientCapacity(usize, usize), // (number of unselected, tried to select)
}

impl Display for SteganographyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::InsufficientCapacity(unselected_num, tried_to_select) => {
                write!(
                    f,
                    "Tried to select {tried_to_select} planes when there were only {unselected_num} left."
                )
            }
        }
    }
}

impl error::Error for SteganographyError {}
