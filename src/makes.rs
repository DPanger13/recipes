mod api;

use crate::error::VehiclesError;
use crate::makes::api::{fetch_makes, MakesResponse};

#[derive(uniffi::Record)]
pub struct Make {
    pub name: String,
}

#[uniffi::export]
pub fn makes(id: String) -> Result<Vec<Make>, VehiclesError> {
    let result = fetch_makes(id)?;
    Ok(create_makes(result))
}

fn create_makes(response: MakesResponse) -> Vec<Make> {
    response
        .makes
        .iter()
        .map(|make| Make {
            name: make.name.to_owned(),
        })
        .collect()
}
