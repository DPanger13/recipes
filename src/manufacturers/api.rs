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
    pub id: String,

    #[serde(rename(deserialize = "Mfr_CommonName"))]
    pub name: String,
}

pub fn fetch_manufacturers() -> reqwest::Result<ManufacturersResponse> {
    let full_url = VEHICLES_API.to_owned() + "/vehicles/GetAllManufacturers?format=json";
    reqwest::blocking::get(full_url)?.json::<ManufacturersResponse>()
}
