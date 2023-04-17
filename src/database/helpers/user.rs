#![allow(dead_code)]
use crate::{
    find_one,
    models::{
        token::{JWT, TOKEN},
        user::ACCOUNT,
    },
    update_one,
};
use mongodb::bson::{doc, oid::ObjectId};
use rocket::http::Status;

use crate::database::repository::Repository;

impl Repository {
    pub async fn get_account(&self, number: String) -> Result<ACCOUNT, Status> {
        find_one!(&self.account, None, ("number", &number))
    }

    pub async fn get_account_from_id(&self, id: &str) -> Result<ACCOUNT, Status> {
        find_one!(&self.account, None, _id => id)
    }

    pub async fn login_account(&self, sub: ObjectId, number: String) -> Result<String, Status> {
        let token = TOKEN::new(sub.to_string(), "User".to_string()).unwrap();
        let mut jwt = JWT::token_to_jwt(&token, &self.keys)?;
        let id = self.insert_token(jwt.clone(), sub).await?;
        jwt.set_id(id);
        let update = doc! {
            "$set": {
                "token": id
            }
        };
        match update_one!(self.account, update, None, ("number", &number)) {
            Ok(result) => {
                if result.modified_count == 1 {
                    return Ok(jwt.encrypt_details(sub.to_string(), &self.keys)?);
                }
                Err(Status::InternalServerError)
            }
            Err(error) => return Err(error),
        }
    }

    pub async fn credit_amount(&self, number: String, amount: i64) -> Result<(), Status> {
        let update = doc! {
            "$inc": {
                "balance": amount
            }
        };
        match update_one!(&self.account, update, None, ("number", number)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Status::InternalServerError),
        }
    }

    pub async fn debit_amount(&self, number: String, amount: i64) -> Result<(), Status> {
        let update = doc! {
            "$inc": {
                "balance": -amount
            }
        };
        match update_one!(&self.account, update, None, ("number", number)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Status::InternalServerError),
        }
    }
}
