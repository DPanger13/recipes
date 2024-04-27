use crate::error::VehiclesError;
use crate::manufacturers::api::{fetch_manufacturers, ManufacturersResponse};

mod api;

#[derive(uniffi::Record)]
pub struct Manufacturer {
    pub id: String,
    pub name: String,
}

#[uniffi::export]
pub fn manufacturers() -> Result<Vec<Manufacturer>, VehiclesError> {
    let result = fetch_manufacturers()?;
    Ok(create_manufacturers(result))
}

fn create_manufacturers(response: ManufacturersResponse) -> Vec<Manufacturer> {
    response
        .manufacturers
        .iter()
        .map(|manufacturer| {
            let name = match &manufacturer.name {
                None => "Unknown".to_string(),
                Some(name) => name.to_string(),
            };
            Manufacturer {
                id: manufacturer.id.to_string(),
                name: name.to_owned(),
            }
        })
        .collect()
}
