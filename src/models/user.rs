use crate::{
    models::helpers::common::timestamp,
    pwd,
    utilities::crypto::{hasher, Generator},
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

fn default_balance() -> i64 {
    0
}

fn default_token() -> Option<ObjectId> {
    None
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ACCOUNT {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default = "default_balance")]
    pub balance: i64,
    #[serde(skip_serializing_if = "Option::is_none", default = "default_token")]
    pub token: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<i16>,
}

impl ACCOUNT {
    pub fn new(
        id: Option<ObjectId>,
        name: String,
        password: String,
        number: Option<String>,
    ) -> Self {
        Self {
            id,
            name: Some(name),
            password,
            number: if number.is_none() {
                Some(hasher(timestamp().to_string()))
            } else {
                number
            },
            balance: default_balance(),
            token: None,
            pin: Some(Generator::generate_number_i16(1000, 9999)),
        }
    }

    pub fn generate_number(&mut self) -> &mut Self {
        if self.number.is_none() {
            self.number = Some(hasher(timestamp().to_string()))
        }
        self
    }

    pub fn generate_pin(&mut self) -> &mut Self {
        if self.pin.is_none() {
            self.pin = Some(Generator::generate_number_i16(1000, 9999))
        }
        self
    }
}
pwd!(ACCOUNT);
