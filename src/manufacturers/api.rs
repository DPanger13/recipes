use crate::api::VEHICLES_API;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ManufacturersResponse {
    #[serde(rename(deserialize = "Results"))]
    pub manufacturers: Vec<ManufacturerResponse>,
}

#[derive(Deserialize)]
pub struct ManufacturerResponse {
    #[serde(rename(deserialize = "Mfr_ID"))]
    pub id: u64,

    #[serde(rename(deserialize = "Mfr_CommonName"))]
    pub name: Option<String>,
}

pub fn fetch_manufacturers() -> reqwest::Result<ManufacturersResponse> {
    let full_url = VEHICLES_API.to_owned() + "/vehicles/GetAllManufacturers?format=json";
    reqwest::blocking::get(full_url)?.json::<ManufacturersResponse>()
}

pub fn fetch_manufacturers_by_name(name: String) -> reqwest::Result<ManufacturersResponse> {
    let full_url = VEHICLES_API.to_owned() + "/vehicles/GetManufacturerDetails/" + &name;
    reqwest::blocking::get(full_url)?.json::<ManufacturersResponse>()
}
