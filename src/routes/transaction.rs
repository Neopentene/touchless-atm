use crate::{
    check_if_401, check_result,
    database::repository::Repository,
    models::{
        handlers::Response,
        token::{Type, TOKEN},
        transaction::{TxnType, TRANSACTION},
    },
};
use rocket::{http::Status, serde::json::Json, State};

#[post("/account/txn/create", data = "<txn>")]
pub async fn create_txn_account(
    token: TOKEN,
    db: &State<Repository>,
    txn: Json<TRANSACTION>,
) -> Result<Response<String>, Status> {
    check_if_401!(!Type::ACCOUNT.cmp(&token.role.value()));
    check_if_406!(txn.atm.is_none());
    let account = db.get_account_from_id(&token.sub).await?;
    let number = account.number.as_ref().unwrap();
    let atm = db.get_atm(txn.atm.as_ref().unwrap().to_owned()).await?;
    let amount = txn.amount;
    let mut txn = TRANSACTION::new(number, &atm.name, "DEBIT", amount);
    check_result!(
        db.reject_all_pending_txn("account", &number).await,
        "Reject All Pending Txn"
    );
    match txn.is_valid(account.balance) {
        true => {
            let otp = txn.generate_otp().otp.unwrap();

            check_ok_500!(db.create_txn(account.id, txn).await, "Creating Transaction")?;
            Ok(Response::<String>::new()
                .message("Transaction Created".to_string())
                .data(otp.to_string())
                .status(Status::Ok)
                .clone())
        }
        false => {
            check_ok_500!(db.create_txn(account.id, txn).await, "Creating Transaction")?;
            Ok(Response::<String>::new()
                .fail()
                .error("Insufficient Balance".to_string())
                .status(Status::NotAcceptable)
                .clone())
        }
    }
}

#[post("/atm/txn/create", data = "<txn>")]
pub async fn create_txn_atm(
    token: TOKEN,
    db: &State<Repository>,
    txn: Json<TRANSACTION>,
) -> Result<Response<String>, Status> {
    let txn_type = txn.txn_type.to_string();
    check_if_401!(!Type::ATM.cmp(&token.role.value()));
    check_if_406!(txn.account.is_none());
    let account = db
        .get_account(txn.account.as_ref().unwrap().to_owned())
        .await?;
    let number = account.number.as_ref().unwrap();
    let atm = db.get_atm_from_id(&token.sub).await?;
    let amount = txn.amount;
    let mut txn = TRANSACTION::new(number, &atm.name, &txn_type, amount);
    check_result!(
        db.reject_all_pending_txn("atm", &atm.name).await,
        "Reject All Pending Txn"
    );
    match txn.is_valid(account.balance) {
        true => {
            let otp = txn.generate_otp().otp.unwrap();
            check_ok_500!(db.create_txn(account.id, txn).await, "Creating Transaction")?;
            Ok(Response::<String>::new()
                .message("Transaction Created".to_string())
                .data(otp.to_string())
                .status(Status::Ok)
                .clone())
        }
        false => {
            check_ok_500!(db.create_txn(account.id, txn).await, "Creating Transaction")?;
            Ok(Response::<String>::new()
                .fail()
                .error("Insufficient Balance".to_string())
                .status(Status::NotAcceptable)
                .clone())
        }
    }
}

#[post("/account/txn/confirm/<pin>", data = "<txn>")]
pub async fn confirm_txn_account(
    token: TOKEN,
    db: &State<Repository>,
    txn: Json<TRANSACTION>,
    pin: String,
) -> Result<Response<String>, Status> {
    check_if_401!(!Type::ACCOUNT.cmp(&token.role.value()));
    check_if_406!(txn.otp.is_none());
    let otp = txn.otp.unwrap();

    let pin = check_ok_406!(pin.parse::<i16>())?;
    let account = db.get_account_from_id(&token.sub).await?;
    let number = account.number.as_ref().unwrap().to_owned();
    let mut txn = db.get_pending_txn("account", &number).await?;

    check_if_406!(!pin == account.pin.unwrap());
    check_if_406!(otp != txn.otp.unwrap());
    match txn.is_valid(account.balance) {
        true => {
            match txn.txn_type {
                TxnType::DEBIT => db.debit_amount(number.to_owned(), txn.amount).await?,
                TxnType::CREDIT => db.credit_amount(number.to_owned(), txn.amount).await?,
            }
            check_result!(db.confirm_txn("account", &number, "complete").await, "Txn");
            Ok(Response::<String>::new()
                .success()
                .status(Status::Ok)
                .clone())
        }
        false => {
            check_result!(db.confirm_txn("account", &number, "rejected").await, "Txn");
            Ok(Response::<String>::new()
                .fail()
                .error("Account Balance Insufficient or Transaction Expired".to_string())
                .status(Status::NotAcceptable)
                .clone())
        }
    }
}

#[post("/atm/txn/confirm/<pin>", data = "<txn>")]
pub async fn confirm_txn_atm(
    token: TOKEN,
    db: &State<Repository>,
    txn: Json<TRANSACTION>,
    pin: String,
) -> Result<Response<String>, Status> {
    check_if_401!(!Type::ATM.cmp(&token.role.value()));
    check_if_406!(txn.otp.is_none());
    let otp = txn.otp.unwrap();

    let pin = check_ok_406!(pin.parse::<i16>())?;
    let atm = db.get_atm_from_id(&token.sub).await?;
    let mut txn = db.get_pending_txn("atm", &atm.name).await?;
    let account = db
        .get_account(txn.account.as_ref().unwrap().to_owned())
        .await?;
    let number = account.number.as_ref().unwrap().to_owned();

    check_if_406!(!pin == account.pin.unwrap());
    check_if_406!(otp != txn.otp.unwrap());
    match txn.is_valid(account.balance) {
        true => {
            match txn.txn_type {
                TxnType::DEBIT => db.debit_amount(number.to_owned(), txn.amount).await?,
                TxnType::CREDIT => db.credit_amount(number.to_owned(), txn.amount).await?,
            }
            check_result!(db.confirm_txn("account", &number, "complete").await, "Txn");
            Ok(Response::<String>::new()
                .success()
                .status(Status::Ok)
                .clone())
        }
        false => {
            check_result!(db.confirm_txn("account", &number, "rejected").await, "Txn");
            Ok(Response::<String>::new()
                .fail()
                .status(Status::NotAcceptable)
                .clone())
        }
    }
}

#[get("/account/txn/otp")]
pub async fn get_otp_account(
    token: TOKEN,
    db: &State<Repository>,
) -> Result<Response<i16>, Status> {
    check_if_401!(!Type::ACCOUNT.cmp(&token.role.value()));
    let account = db.get_account_from_id(&token.sub).await?;
    let number = account.number.unwrap();
    let txn = db.get_pending_txn("account", &number).await?;
    Ok(Response::<i16>::new()
        .success()
        .data(txn.otp.unwrap())
        .status(Status::Found)
        .clone())
}

#[get("/atm/txn/otp")]
pub async fn get_otp_atm(token: TOKEN, db: &State<Repository>) -> Result<Response<i16>, Status> {
    check_if_401!(!Type::ATM.cmp(&token.role.value()));
    let atm = db.get_atm_from_id(&token.sub).await?;
    let txn = db.get_pending_txn("atm", &atm.name).await?;
    Ok(Response::<i16>::new()
        .success()
        .data(txn.otp.unwrap())
        .status(Status::Found)
        .clone())
}

#[get("/atm/txn/status")]
pub async fn get_txn_status_atm(
    token: TOKEN,
    db: &State<Repository>,
) -> Result<Response<String>, Status> {
    check_if_401!(!Type::ATM.cmp(&token.role.value()));
    let atm = db.get_atm_from_id(&token.sub).await?;
    let txn = db.get_recent_txn("atm", &atm.name).await?;
    Ok(Response::<String>::new()
        .success()
        .data(txn.status.value())
        .status(Status::Found)
        .clone())
}

#[get("/atm/txn/recent/value")]
pub async fn get_txn_recent_value(
    token: TOKEN,
    db: &State<Repository>,
) -> Result<Response<i64>, Status> {
    check_if_401!(!Type::ATM.cmp(&token.role.value()));
    let atm = db.get_atm_from_id(&token.sub).await?;
    let txn = db.get_recent_txn("atm", &atm.name).await?;
    Ok(Response::<i64>::new()
        .success()
        .data(txn.amount)
        .status(Status::Found)
        .clone())
}

#[get("/atm/txn/status/reject")]
pub async fn reject_atm_txn(
    token: TOKEN,
    db: &State<Repository>,
) -> Result<Response<String>, Status> {
    check_if_401!(!Type::ATM.cmp(&token.role.value()));
    let atm = db.get_atm_from_id(&token.sub).await?;
    check_ok_406!(db.reject_all_pending_txn("atm", &atm.name).await)?;
    Ok(Response::<String>::new()
        .success()
        .status(Status::Ok)
        .clone())
}
