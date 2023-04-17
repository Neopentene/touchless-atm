#![allow(dead_code)]
use std::str::FromStr;

use crate::{
    find_one, generate_one,
    models::{
        helpers::common::timestamp_millis,
        transaction::{TxnStatus, TRANSACTION},
    },
    update_many, update_one,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOneOptions,
};
use rocket::http::Status;

use crate::database::repository::Repository;

impl Repository {
    // Throws 400, 409 and 500
    pub async fn create_txn(&self, id: Option<ObjectId>, data: TRANSACTION) -> Result<(), Status> {
        let id = match id {
            Some(result) => result,
            None => return Err(Status::BadRequest),
        };
        let logs = &self.logs;
        let timestamp = timestamp_millis();
        generate_one!(&self.txn, logs, data, None, id, timestamp, "TRANSACTION")
    }

    pub async fn get_recent_txn(&self, field: &str, value: &str) -> Result<TRANSACTION, Status> {
        let options = FindOneOptions::builder()
            .sort(doc! {
                "created": -1
            })
            .build();
        find_one!(&self.txn, options, (field, value))
    }

    pub async fn get_pending_txn(&self, field: &str, value: &str) -> Result<TRANSACTION, Status> {
        find_one!(&self.txn, None, (field, value), ("status", "PENDING"))
    }

    pub async fn reject_all_pending_txn(&self, field: &str, value: &str) -> Result<(), Status> {
        let update = doc! {
            "$set": {
                "status": "REJECTED"
            }
        };
        match update_many!(
            &self.txn,
            update,
            None,
            (field, value),
            ("status", "PENDING")
        ) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub async fn confirm_txn(&self, field: &str, value: &str, status: &str) -> Result<(), Status> {
        let status = TxnStatus::from_str(status).unwrap().to_string();
        let update = doc! {
            "$set": {
                "status": status
            }
        };
        match update_one!(
            &self.txn,
            update,
            None,
            (field, value),
            ("status", "PENDING")
        ) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
}
