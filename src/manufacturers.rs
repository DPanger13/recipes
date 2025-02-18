use crate::error::VehiclesError;
use crate::manufacturers::api::{
    fetch_manufacturers, fetch_manufacturers_by_name, ManufacturersResponse,
};

mod api;

#[derive(uniffi::Record)]
pub struct Manufacturers {
    pub manufacturers: Vec<Manufacturer>,
    pub has_more: bool,
}

#[derive(uniffi::Record)]
pub struct Manufacturer {
    pub id: String,
    pub name: String,
}

#[uniffi::export]
pub fn manufacturers(page: i32) -> Result<Manufacturers, VehiclesError> {
    let result = fetch_manufacturers(page)?;
    let manufacturers = create_manufacturers(result);
    let has_more = manufacturers.len() == 100;
    Ok(Manufacturers {
        manufacturers,
        has_more,
    })
}

#[uniffi::export]
pub fn search_manufacturers(name: String, page: i32) -> Result<Manufacturers, VehiclesError> {
    let result = fetch_manufacturers_by_name(name, page)?;
    let manufacturers = create_manufacturers(result);
    let has_more = manufacturers.len() == 100;
    Ok(Manufacturers {
        manufacturers,
        has_more,
    })
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
