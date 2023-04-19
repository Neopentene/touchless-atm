use super::helpers::common::timestamp_millis;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::{num::Wrapping, str::FromStr};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Type {
    ADMIN,
    ACCOUNT,
    ATM,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JWT {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // key
    pub jwt: String,
    pub created: DateTime, // For expiry
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TOKEN {
    pub sub: String,
    pub role: Type,
    pub exp: i64,
}

impl TOKEN {
    pub const ENCODING_ERROR: &str = "Failed to Encode JWT";
    pub const DECODING_ERROR: &str = "Failed to Decode JWT";
    pub const INTERNAL_SERVER_ERROR: &str = "Internal Server Failure";
    pub const UNAUTHORIZED_ERROR: &str = "Unauthorized";
    pub const NOT_FOUND: &str = "Not Found";
    pub const BEARER: &str = "Bearer ";
    pub const DEFAULT_EXPIRY: i64 = 86_400_000;

    pub fn new(sub: String, role: String) -> Result<TOKEN, String> {
        let timestamp = timestamp_millis();
        let exp = Wrapping(timestamp + Self::DEFAULT_EXPIRY);
        let role = Type::from_str(&role)?;

        Ok(TOKEN {
            sub,
            role,
            exp: exp.0,
        })
    }
}

impl JWT {
    pub fn new(jwt: String) -> Self {
        let created = DateTime::from_millis(timestamp_millis());
        Self {
            id: None,
            jwt,
            created,
        }
    }

    pub fn set_id(&mut self, id: ObjectId) -> &mut Self {
        self.id = Some(id);
        self
    }
}
