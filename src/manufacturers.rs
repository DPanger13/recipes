use crate::error::VehiclesError;
use crate::manufacturers::api::{fetch_manufacturers, ManufacturersResponse};

mod api;

#[derive(uniffi::Record)]
pub struct Manufacturer {
    id: String,
    name: String,
}

#[uniffi::export]
pub fn manufacturers() -> Result<Vec<Manufacturer>, VehiclesError> {
    let result = fetch_manufacturers();
    match result {
        Ok(response) => Ok(create_manufacturers(response)),
        Err(_) => Err(VehiclesError::Fetch),
    }
}

fn create_manufacturers(response: ManufacturersResponse) -> Vec<Manufacturer> {
    response
        .manufacturers
        .iter()
        .map(|manufacturer| Manufacturer {
            id: manufacturer.id.to_owned(),
            name: manufacturer.name.to_owned(),
        })
        .collect()
}
