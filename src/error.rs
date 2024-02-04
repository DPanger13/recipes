use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, uniffi::Enum)]
pub enum VehiclesError {
    Fetch,
}

impl Error for VehiclesError {}

impl fmt::Display for VehiclesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VehiclesError::Fetch => write!(f, "Failed to fetch data"),
        }
    }
}
