use crate::{
    check_if_400, check_if_401, check_ok_401,
    database::repository::Repository,
    models::{
        admin::{Role, ADMIN},
        atm::ATM,
        handlers::Response,
        token::{Type, TOKEN},
        user::ACCOUNT,
    },
};
use rocket::{http::Status, serde::json::Json, State};

#[post("/admin/create", data = "<new_admin>")]
pub async fn create_admin(
    token: TOKEN,
    db: &State<Repository>,
    new_admin: Json<ADMIN>,
) -> Result<Response<String>, Status> {
    check_if_401!(!Type::ADMIN.cmp(&token.role.value()));

    let admin = check_ok_401!(db.get_admin_from_id(&token.sub).await)?;

    let mut data = match new_admin.0.validate() {
        Ok(data) => data.clone(),
        Err(_) => return Err(Status::BadRequest),
    };

    match (admin.role, data.role) {
        (Some(admin_role), Some(data_role)) => check_if_401!(admin_role < data_role),
        _ => return Err(Status::InternalServerError),
    };
    data.hash_password()?;

    match db.create_admin(admin.id, data.clone()).await {
        Ok(_) => {
            return Ok(Response::<String>::new()
                .message(format!("Created Admin: {}", data.username))
                .status(Status::Created)
                .clone())
        }
        Err(_) => return Err(Status::InternalServerError),
    }
}

#[post("/atm/create", data = "<atm>")]
pub async fn create_atm(
    token: TOKEN,
    db: &State<Repository>,
    atm: Json<ATM>,
) -> Result<Response<String>, Status> {
    check_if_401!(!Type::ADMIN.cmp(&token.role.value()));

    let admin = check_ok_401!(db.get_admin_from_id(&token.sub).await)?;
    check_if_401!(admin.role.unwrap() < Role::SUPERVISOR);

    let mut data = atm.0;
    data.hash_password()?;

    match db.create_atm(admin.id, data.clone()).await {
        Ok(_) => {
            return Ok(Response::<String>::new()
                .message(format!("Added ATM: {}", data.name))
                .status(Status::Created)
                .clone())
        }
        Err(_) => return Err(Status::InternalServerError),
    }
}

#[post("/account/create", data = "<account>")]
pub async fn create_account(
    token: TOKEN,
    db: &State<Repository>,
    account: Json<ACCOUNT>,
) -> Result<Response<ACCOUNT>, Status> {
    check_if_401!(!Type::ADMIN.cmp(&token.role.value()));
    check_if_400!(account.name.is_none());

    let admin = check_ok_401!(db.get_admin_from_id(&token.sub).await)?;

    let mut data = match account.0.validate() {
        Ok(data) => data.clone(),
        Err(_) => return Err(Status::BadRequest),
    };
    data.generate_number();
    data.generate_pin();
    data.hash_password()?;

    let name = data.name.as_ref().unwrap().to_owned();

    match db.create_account(admin.id, data.clone()).await {
        Ok(_) => {
            data.password = "$SEALED$".to_string();
            return Ok(Response::<ACCOUNT>::new()
                .message(format!("Created Account: {}", name))
                .data(data)
                .status(Status::Created)
                .clone());
        }
        Err(_) => return Err(Status::InternalServerError),
    }
}
