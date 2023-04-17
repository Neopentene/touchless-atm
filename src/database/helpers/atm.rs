use crate::{
    find_one,
    models::{
        atm::ATM,

        token::{JWT, TOKEN},
    },
    update_one,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
};
use rocket::http::Status;

use crate::database::repository::Repository;

impl Repository {
    pub async fn get_atm(&self, name: String) -> Result<ATM, Status> {
        find_one!(&self.atm, None, ("name", &name))
    }

    pub async fn get_atm_from_id(&self, id: &str) -> Result<ATM, Status> {
        find_one!(&self.atm, None, _id => id)
    }

    pub async fn login_atm(&self, sub: ObjectId, name: String) -> Result<String, Status> {
        let token = TOKEN::new(sub.to_string(), "Atm".to_string()).unwrap();
        let mut jwt = JWT::token_to_jwt(&token, &self.keys)?;
        let id = self.insert_token(jwt.clone(), sub).await?;
        jwt.set_id(id);
        let update = doc! {
            "$set": {
                "token": id
            }
        };
        match update_one!(self.atm, update, None, ("name", &name)) {
            Ok(result) => {
                if result.modified_count == 1 {
                    return Ok(jwt.encrypt_details(sub.to_string(), &self.keys)?);
                }
                Err(Status::InternalServerError)
            }
            Err(error) => return Err(error),
        }
    }
}
