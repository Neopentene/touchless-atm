use crate::{
    database::repository::Repository,
    find_one, insert_one, log_action,
    models::{helpers::common::timestamp, token::JWT},
    option,
};
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;

impl Repository {
    pub async fn get_token(&self, id: &str) -> Result<JWT, Status> {
        find_one!(&self.token, None, _id => id)
    }

    pub async fn insert_token(&self, token: JWT, creator: ObjectId) -> Result<ObjectId, Status> {
        let id = insert_one!(&self.token, token, None)?.inserted_id;
        let id =
            option!(val -> id.as_object_id(); {val} | {return Err(Status::InternalServerError)});
        log_action!(&self.logs, creator, timestamp(), "TOKEN", id);
        Ok(id)
    }
}
