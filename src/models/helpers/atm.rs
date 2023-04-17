use crate::models::atm::{Field, Latitude, Location, Longitude, ATM};
use serde::{
    de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::{Serialize, SerializeStruct},
};
use std::fmt;

impl Latitude {
    pub fn to_decimal(&self) -> f32 {
        self.0 as f32 + (self.1 as f32 / 60.0) + (self.2 / 3600.0)
    }
}

impl Longitude {
    pub fn to_decimal(&self) -> f32 {
        self.0 as f32 + (self.1 as f32 / 60.0) + (self.2 / 3600.0)
    }
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("location", 2)?;
        state.serialize_field("latitude", &self.0.to_decimal())?;
        state.serialize_field("longitude", &self.1.to_decimal())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LocationVisitor;

        impl<'de> Visitor<'de> for LocationVisitor {
            type Value = Location;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Location")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Location, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let latitude_decimal_degrees = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let longitude_decimal_degrees = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Location::new(latitude_decimal_degrees, longitude_decimal_degrees).unwrap())
            }

            fn visit_map<V>(self, mut map: V) -> Result<Location, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut latitude_decimal_degrees: Option<f32> = None;
                let mut longitude_decimal_degrees: Option<f32> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Latitude => {
                            if latitude_decimal_degrees.is_some() {
                                return Err(de::Error::duplicate_field("latitude"));
                            }
                            latitude_decimal_degrees = Some(map.next_value()?);
                        }
                        Field::Longitude => {
                            if longitude_decimal_degrees.is_some() {
                                return Err(de::Error::duplicate_field("longitude"));
                            }
                            longitude_decimal_degrees = Some(map.next_value()?);
                        }
                    }
                }
                let latitude_decimal_degrees =
                    latitude_decimal_degrees.ok_or_else(|| de::Error::missing_field("latitude"))?;
                let longitude_decimal_degrees = longitude_decimal_degrees
                    .ok_or_else(|| de::Error::missing_field("longitude"))?;
                Ok(Location::new(latitude_decimal_degrees, longitude_decimal_degrees).unwrap())
            }
        }

        const FIELDS: &'static [&'static str] = &["latitude", "longitude"];
        deserializer.deserialize_struct("Location", FIELDS, LocationVisitor)
    }
}

impl Clone for ATM {
    fn clone(&self) -> Self {
        let latitude_decimal_degrees = self.coordinates.0.to_decimal();
        let longitude_decimal_degrees = self.coordinates.1.to_decimal();

        Self {
            id: self.id,
            name: self.name.to_owned(),
            branch: self.branch.to_owned(),
            address: self.address.to_owned(),
            token: self.token.to_owned(),
            coordinates: Location::new(latitude_decimal_degrees, longitude_decimal_degrees)
                .unwrap(),
            password: self.password.to_owned(),
        }
    }
}
