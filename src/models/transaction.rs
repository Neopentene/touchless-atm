use std::str::FromStr;

use crate::utilities::crypto::Generator;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use super::helpers::common::timestamp_millis;

#[derive(Debug, Deserialize, Serialize)]
pub enum TxnStatus {
    PENDING,  // Ongoing... wait for otp confirmation
    EXPIRED,  // Timeout
    COMPLETE, // Successful
    REJECTED, // Balance Insufficient
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum TxnType {
    DEBIT,
    CREDIT,
}

fn default_amount() -> i64 {
    0
}

fn default_status() -> TxnStatus {
    TxnStatus::PENDING
}

fn default_txn_type() -> TxnType {
    TxnType::DEBIT
}

fn default_creation_time() -> DateTime {
    DateTime::from_millis(timestamp_millis())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TRANSACTION {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atm: Option<String>,
    #[serde(default = "default_amount")]
    pub amount: i64,
    #[serde(default = "default_txn_type")]
    pub txn_type: TxnType,
    #[serde(default = "default_status")]
    pub status: TxnStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp: Option<i16>,
    #[serde(default = "default_creation_time")]
    pub created: DateTime,
}

impl TRANSACTION {
    pub const DEFAULT_EXPIRY: i64 = 3600000;

    pub fn new(account: &str, atm: &str, txn_type: &str, amount: i64) -> Self {
        let created = DateTime::from_millis(timestamp_millis());
        Self {
            id: None,
            account: Some(account.to_string()),
            atm: Some(atm.to_string()),
            amount,
            txn_type: TxnType::from_str(txn_type).unwrap(),
            status: TxnStatus::PENDING,
            otp: Some(Generator::generate_number_i16(1000, 9999)),
            created,
        }
    }

    pub fn generate_otp(&mut self) -> &mut Self {
        self.otp = Some(Generator::generate_number_i16(1000, 9999));
        self
    }
}
