#![allow(dead_code)]
use crate::{
    create_one,
    database::repository::Repository,
    find_one, insert_one,
    models::{
        admin::ADMIN,
        atm::ATM,
        helpers::common::timestamp_millis,
        token::{JWT, TOKEN},
        user::ACCOUNT,
    },
    update_one,
};
use mongodb::bson::{doc, oid::ObjectId};
use rocket::http::Status;

impl Repository {
    // Throws 400, 409 and 500
    pub async fn create_admin(&self, id: Option<ObjectId>, data: ADMIN) -> Result<(), Status> {
        let id = match id {
            Some(result) => result,
            None => return Err(Status::BadRequest),
        };
        let logs = &self.logs;
        let (sub, timestamp) = (data.username.to_owned(), timestamp_millis());
        create_one!(&self.admin, logs, data, None, id, timestamp, sub, "Admin")
    }

    // Throws 400, 409 and 500
    pub async fn create_atm(&self, id: Option<ObjectId>, data: ATM) -> Result<(), Status> {
        let id = match id {
            Some(result) => result,
            None => return Err(Status::BadRequest),
        };
        let (sub, timestamp) = (data.name.to_owned(), timestamp_millis());
        create_one!(&self.atm, &self.logs, data, None, id, timestamp, sub, "Atm")
    }

    pub async fn create_account(&self, id: Option<ObjectId>, data: ACCOUNT) -> Result<(), Status> {
        let id = match id {
            Some(result) => result,
            None => return Err(Status::BadRequest),
        };
        let logs = &self.logs;
        let sub = data.number.as_ref().unwrap().to_owned();
        let timestamp = timestamp_millis();
        create_one!(&self.account, logs, data, None, id, timestamp, sub, "User")
    }

    // Throws 404 and 500
    pub async fn get_admin(&self, username: String) -> Result<ADMIN, Status> {
        find_one!(&self.admin, None, ("username", username))
    }

    pub async fn get_admin_from_id(&self, id: &str) -> Result<ADMIN, Status> {
        find_one!(&self.admin, None, _id => id)
    }

    // Throws 404 and 500
    pub async fn login_admin(&self, sub: ObjectId, username: String) -> Result<String, Status> {
        let time = timestamp_millis();
        let token = TOKEN::new(sub.to_string(), "Admin".to_string()).unwrap();

        println!("Token Created in time {}", timestamp_millis() - time);
        let mut jwt = JWT::token_to_jwt(&token, &self.keys)?;
        let id = self.insert_token(jwt.clone(), sub).await?;

        println!("Inserted Token in time {}", timestamp_millis() - time);
        jwt.set_id(id);
        let update = doc! {
            "$set": {
                "token": id
            }
        };
        match update_one!(self.admin, update, None, ("username", &username)) {
            Ok(result) => {
                if result.modified_count == 1 {
                    println!("Updated in time {}", timestamp_millis() - time);
                    return Ok(jwt.encrypt_details(sub.to_string(), &self.keys)?);
                }
                Err(Status::InternalServerError)
            }
            Err(error) => return Err(error),
        }
    }
}
