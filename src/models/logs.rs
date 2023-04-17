use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Nature {
    CREATION(CREATION),
    GENERATION(GENERATION),
}

pub enum Type {
    ADMIN,
    ACCOUNT,
    ATM,
    TRANSACTION,
    TOKEN,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CREATION {
    pub affected_id: ObjectId,
    pub sub: String,
    pub nature: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GENERATION {
    pub affected_id: ObjectId,
    pub nature: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LOG {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub timestamp: Option<i64>,
    pub creator: Option<ObjectId>,
    pub role: Option<String>,
    pub change: Option<Nature>,
}

impl LOG {
    pub fn new() -> Self {
        Self {
            id: None,
            timestamp: None,
            creator: None,
            role: None,
            change: None,
        }
    }

    pub fn timestamp(&mut self, timestamp: i64) -> &mut Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn creator(&mut self, creator: ObjectId) -> &mut Self {
        self.creator = Some(creator);
        self
    }

    pub fn role(&mut self, role: &str) -> &mut Self {
        self.role = Some(Type::from_str(role).unwrap().to_string());
        self
    }

    pub fn creation(&mut self, affected_id: ObjectId, sub: String) -> &mut Self {
        self.change = Some(Nature::CREATION(CREATION {
            affected_id,
            sub,
            nature: "CREATION".to_string(),
        }));
        self
    }

    pub fn generation(&mut self, affected_id: ObjectId) -> &mut Self {
        self.change = Some(Nature::GENERATION(GENERATION {
            affected_id,
            nature: "GENERATION".to_string(),
        }));
        self
    }

    pub fn build(&self) -> Self {
        Self {
            id: self.id,
            timestamp: self.timestamp,
            creator: self.creator,
            role: self.role.to_owned(),
            change: self.change.to_owned(),
        }
    }
}
