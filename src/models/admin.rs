use crate::pwd;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
pub enum Role {
    STAFF,
    COORIDNATOR,
    SUPERVISOR,
    MANAGER,
    EXECUTIVE,
    MANAGEMENT,
}

impl Role {
    pub fn value(&self) -> i8 {
        match self {
            Role::STAFF => 1,
            Role::COORIDNATOR => 2,
            Role::SUPERVISOR => 3,
            Role::MANAGER => 4,
            Role::EXECUTIVE => 5,
            Role::MANAGEMENT => 6,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ADMIN {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // DB only use
    pub username: String, // Unique
    pub password: String, // Maybe hashed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<ObjectId>, // Token
}

impl ADMIN {
    pub fn new(
        id: Option<ObjectId>,
        username: String,
        password: String,
        role: Option<Role>,
    ) -> Self {
        Self {
            id,
            username,
            password,
            role,
            token: None,
        }
    }
}
pwd!(ADMIN);
