use crate::pwd;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
pub enum Field {
    Latitude,
    Longitude,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Longitude(pub i32, pub i32, pub f32);

#[derive(Debug, Deserialize, Serialize)]
pub struct Latitude(pub i32, pub i32, pub f32);

#[derive(Debug)]
pub struct Location(pub Latitude, pub Longitude);

#[derive(Debug, Deserialize, Serialize)]
pub struct ATM {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub branch: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<ObjectId>,
    pub coordinates: Location,
    pub password: String,
}

fn destructure_locale(decimal_degrees: f32, locale_type: &str) -> Result<(i32, i32, f32), String> {
    let locale: HashMap<&str, f32> = HashMap::from([("LATITUDE", 90.0), ("LONGITUDE", 180.0)]);
    let dec = decimal_degrees.abs();

    if dec >= *locale.get(locale_type).unwrap() {
        return Err(format!("Invalid {}", locale_type));
    }

    let minutes = dec.fract() * 60.0;
    let seconds = minutes.fract() * 60.0;

    Ok((decimal_degrees as i32, minutes.abs() as i32, seconds.abs()))
}

impl Latitude {
    pub fn new(decimal_degrees: f32) -> Result<Latitude, String> {
        let (degrees, minutes, seconds) = destructure_locale(decimal_degrees, "LATITUDE")?;
        Ok(Latitude(degrees, minutes, seconds))
    }
}

impl Longitude {
    pub fn new(decimal_degrees: f32) -> Result<Longitude, String> {
        let (degrees, minutes, seconds) = destructure_locale(decimal_degrees, "LONGITUDE")?;
        Ok(Longitude(degrees, minutes, seconds))
    }
}

impl Location {
    pub fn new(
        latitude_decimal_degrees: f32,
        longitude_decimal_degrees: f32,
    ) -> Result<Location, String> {
        let (latitude, longitude) = match (
            Latitude::new(latitude_decimal_degrees),
            Longitude::new(longitude_decimal_degrees),
        ) {
            (Ok(latitude), Ok(longitude)) => (latitude, longitude),
            (Err(value), Ok(_)) | (Ok(_), Err(value)) => return Err(value.to_owned()),
            (Err(_), Err(_)) => return Err(String::from("Invalid Latitude and Longitude")),
        };

        return Ok(Location(latitude, longitude));
    }
}

impl ATM {
    pub fn new(
        id: Option<ObjectId>,
        name: String,
        branch: String,
        address: String,
        location: Location,
        password: String,
    ) -> Self {
        Self {
            id,
            name,
            branch,
            address,
            token: None,
            coordinates: location,
            password,
        }
    }
}
pwd!(ATM);
