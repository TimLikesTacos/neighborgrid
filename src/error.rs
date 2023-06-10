use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum GridError {
    IndexOutOfBounds,
    RowSizeMismatch,
    InvalidSize,
    ExcessiveSize,
    InvalidDivisionSize,
}

impl Display for GridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GridError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            GridError::RowSizeMismatch => write!(f, "Row size must match other rows"),
            GridError::InvalidSize => write!(f, "Invalid grid size"),
            GridError::ExcessiveSize => write!(f, "Resulting grid is too large"),
            GridError::InvalidDivisionSize => write!(
                f,
                "Parameter passed if for divisor is either less than 1 or larger than the grid"
            ),
        }
    }
}

impl Error for GridError {}
