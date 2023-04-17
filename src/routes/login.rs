use crate::{
    check_if_400, check_ok_404,
    database::repository::Repository,
    models::{
        admin::ADMIN, atm::ATM, handlers::Response, helpers::common::timestamp_millis,
        user::ACCOUNT,
    },
    utilities::crypto::verify_password,
};
use rocket::{http::Status, serde::json::Json, State};

// #[async_trait]
// #[post("/admin/create", data = "<new_admin>")];
// pub fn create_admin(token)

#[post("/admin/login", data = "<admin>")]
pub async fn login_admin(
    db: &State<Repository>,
    admin: Json<ADMIN>,
) -> Result<Response<String>, Status> {
    let time = timestamp_millis();
    let data = admin.0;
    let admin = db.get_admin(data.username.to_owned()).await?;
    println!("Found Admin in time {}", timestamp_millis() - time);

    let authentication = check_ok_404!(verify_password(&data.password, &admin.password))?;
    println!(
        "Authentication {}? in time {}",
        authentication,
        timestamp_millis() - time
    );
    match authentication {
        true => {
            let token = db.login_admin(admin.id.unwrap(), admin.username).await?;
            println!("Got Token in time {}", timestamp_millis() - time);
            Ok(Response::<String>::new()
                .message("Login Successful".to_string())
                .status(Status::Ok)
                .token(token)
                .clone())
        }
        false => Err(Status::NotFound),
    }
}

#[post("/atm/login", data = "<atm>")]
pub async fn login_atm(db: &State<Repository>, atm: Json<ATM>) -> Result<Response<String>, Status> {
    let data = atm.0;
    let atm = db.get_atm(data.name.to_owned()).await?;

    let authentication = check_ok_404!(verify_password(&data.password, &atm.password))?;
    match authentication {
        true => {
            let token = db.login_atm(atm.id.unwrap(), atm.name).await?;
            Ok(Response::<String>::new()
                .message("Login Successful".to_string())
                .status(Status::Ok)
                .token(token)
                .clone())
        }
        false => Err(Status::NotFound),
    }
}

#[post("/account/login", data = "<account>")]
pub async fn login_account(
    db: &State<Repository>,
    account: Json<ACCOUNT>,
) -> Result<Response<String>, Status> {
    let data = account.0;
    check_if_400!(data.number.is_none());

    let number = data.number.as_ref().unwrap().to_owned();
    let account = db.get_account(number).await?;

    let authentication = check_ok_404!(verify_password(&data.password, &account.password))?;
    match authentication {
        true => {
            let token = db
                .login_account(account.id.unwrap(), account.number.unwrap())
                .await?;
            Ok(Response::<String>::new()
                .message("Login Successful".to_string())
                .status(Status::Ok)
                .token(token)
                .clone())
        }
        false => Err(Status::NotFound),
    }
}
