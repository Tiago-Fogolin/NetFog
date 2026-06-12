use reqwest::blocking::Client;
use crate::external_apis::core::NominatimResponse;
use std::error::Error;

pub fn get_point_from_address(address: String) -> Result<NominatimResponse, Box<dyn Error>>  {
    let client = Client::builder()
        .user_agent("NetFog Library")
        .build()?;

    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",
        address
    );

    let mut response = client
        .get(&url)
        .send()?
        .error_for_status()?
        .json::<Vec<NominatimResponse>>()?;

    if response.is_empty() {
        return Err("Address not found!".into());
    }

    let ret = response.remove(0);

    return Ok(ret);
}
