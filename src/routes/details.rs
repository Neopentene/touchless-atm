use crate::{
    check_if_401,
    database::repository::Repository,
    models::{
        atm::ATM,
        handlers::Response,
        token::{Type, TOKEN},
        user::ACCOUNT,
    },
    seal_details,
};
use rocket::{http::Status, State};

#[get("/get/account")]
pub async fn get_account(
    token: TOKEN,
    db: &State<Repository>,
) -> Result<Response<ACCOUNT>, Status> {
    check_if_401!(!Type::ACCOUNT.cmp(&token.role.value()));
    let mut account = db.get_account_from_id(&token.sub).await?;
    seal_details!(account);
    Ok(Response::<ACCOUNT>::new()
        .data(account)
        .message("Account Details".to_string())
        .status(Status::Ok)
        .clone())
}

#[get("/get/account/<number>")]
pub async fn get_account_admin(
    token: TOKEN,
    db: &State<Repository>,
    number: String,
) -> Result<Response<ACCOUNT>, Status> {
    check_if_401!(!Type::ADMIN.cmp(&token.role.value()));
    let mut account = db.get_account(number).await?;
    seal_details!(account);
    Ok(Response::<ACCOUNT>::new()
        .data(account)
        .message("Account Details".to_string())
        .status(Status::Ok)
        .clone())
}

#[get("/get/atm")]
pub async fn get_atm(token: TOKEN, db: &State<Repository>) -> Result<Response<ATM>, Status> {
    check_if_401!(!Type::ATM.cmp(&token.role.value()));
    let mut atm = db.get_atm_from_id(&token.sub).await?;
    seal_details!(atm);
    Ok(Response::<ATM>::new()
        .data(atm)
        .message("ATM Details".to_string())
        .status(Status::Ok)
        .clone())
}

#[get("/get/atm/<name>")]
pub async fn get_atm_admin(
    token: TOKEN,
    db: &State<Repository>,
    name: String,
) -> Result<Response<ATM>, Status> {
    check_if_401!(!Type::ADMIN.cmp(&token.role.value()));
    let mut atm = db.get_atm(name).await?;
    seal_details!(atm);
    Ok(Response::<ATM>::new()
        .data(atm)
        .message("ATM Details".to_string())
        .status(Status::Ok)
        .clone())
}
