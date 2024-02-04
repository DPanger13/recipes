use crate::api::VEHICLES_API;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MakesResponse {
    #[serde(rename(deserialize = "Results"))]
    pub makes: Vec<MakeResponse>,
}

#[derive(Deserialize)]
pub struct MakeResponse {
    #[serde(rename(deserialize = "Model_Name"))]
    pub name: String,
}

pub fn fetch_makes(id: String) -> reqwest::Result<MakesResponse> {
    let full_url = VEHICLES_API.to_owned() + "/vehicles/GetModelsForMake/" + &id + "?format=json";
    reqwest::blocking::get(full_url)?.json::<MakesResponse>()
}
