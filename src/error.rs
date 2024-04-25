use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, uniffi::Error)]
pub enum VehiclesError {
    Generic {
        source_message: String,
    }
}

impl From<reqwest::Error> for VehiclesError {
    fn from(error: reqwest::Error) -> Self {
        VehiclesError::Generic {
            source_message: error.to_string()
        }
    }
}

impl Error for VehiclesError {}

impl fmt::Display for VehiclesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            VehiclesError::Generic { source_message } => { source_message }
        };
        write!(f, "Failed to fetch data: {}", message)
    }
}
