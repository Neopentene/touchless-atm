use super::common::timestamp_millis;
use crate::database::repository::Repository;
use crate::models::keys::KEY;
use crate::utilities::crypto::{decrypt, encrypt, from_hex};
use crate::{
    models::token::{Type, JWT, TOKEN},
    option,
    utilities::crypto::Generator,
};
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use rocket::http::Status;
use rocket::{
    async_trait,
    request::{FromRequest, Outcome},
    State,
};
use std::str::FromStr;

impl Type {
    pub fn value(&self) -> String {
        match self {
            Type::ADMIN => "ADMIN".to_string(),
            Type::ACCOUNT => "ACCOUNT".to_string(),
            Type::ATM => "ATM".to_string(),
        }
    }

    pub fn cmp(&self, value: &String) -> bool {
        match Self::from_str(value) {
            Ok(value) => {
                if value.value() == self.value() {
                    return true;
                }
                false
            }
            Err(_) => false,
        }
    }
}

impl FromStr for Type {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ADMIN" => Ok(Type::ADMIN),
            "ACCOUNT" => Ok(Type::ACCOUNT),
            "USER" => Ok(Type::ACCOUNT),
            "ATM" => Ok(Type::ATM),
            _ => Err("Invalid Value".to_string()),
        }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::ADMIN => "ADMIN".to_string(),
            Type::ACCOUNT => "ACCOUNT".to_string(),
            Type::ATM => "ATM".to_string(),
        }
    }
}

impl JWT {
    pub fn encrypt_details(&self, sub: String, keys: &KEY) -> Result<String, Status> {
        let id = match self.id {
            Some(id) => id.to_string(),
            None => return Err(Status::NotAcceptable),
        };

        let details = sub + &id; // length 48
        match encrypt(keys.bytes, details, &keys.secret, &keys.seed, keys.rate) {
            Ok(val) => Ok(val),
            Err(error) => {
                println!("Error: {error}");
                Err(Status::InternalServerError)
            }
        }
    }

    pub fn strip_bearer(bearer: &str) -> Result<String, Status> {
        match bearer.starts_with(TOKEN::BEARER) {
            true => Ok(bearer.trim_start_matches(TOKEN::BEARER).to_string()),
            false => Err(Status::NotFound),
        }
    }

    pub fn decrypt_details(details: String, keys: &KEY) -> Result<(String, String), Status> {
        match decrypt(keys.bytes, details, &keys.secret, &keys.seed, keys.rate) {
            Ok(details) => match from_hex(details) {
                Ok(result) => {
                    let details = match String::from_utf8(result) {
                        Ok(result) => result,
                        Err(_) => return Err(Status::NotFound),
                    };
                    let (sub, id) = details.split_at(24);
                    Ok((sub.to_string(), id.to_string()))
                }
                Err(error) => {
                    println!("Error: {error}");
                    Err(Status::NotFound)
                }
            },
            Err(error) => {
                println!("Error: {error}");
                Err(Status::InternalServerError)
            }
        }
    }

    pub fn token_to_jwt(token: &TOKEN, keys: &KEY) -> Result<Self, Status> {
        let header = Header::new(Algorithm::HS256);
        let encoding_key = Generator::generate_token_encoding_key(&keys.secret);
        match encode(&header, token, &encoding_key) {
            Ok(jwt) => Ok(JWT::new(jwt)),
            Err(_) => Err(Status::InternalServerError),
        }
    }

    pub fn token_from_jwt(&self, keys: &KEY) -> Result<TOKEN, Status> {
        let decoding_key = Generator::generate_token_decoding_key(&keys.secret);
        let decoded = decode::<TOKEN>(&self.jwt, &decoding_key, &Validation::new(Algorithm::HS256));
        match decoded {
            Ok(token) => Ok(token.claims),
            Err(_) => Err(Status::InternalServerError),
        }
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for TOKEN {
    type Error = String;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let db = match request.guard::<&State<Repository>>().await {
            Outcome::Success(db) => db,
            _ => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    Self::INTERNAL_SERVER_ERROR.to_string(),
                ))
            }
        };

        let keys = db.keys;

        let bearer = match request.headers().get_one("Authorization") {
            Some(bearer) => bearer.to_string(),
            None => match request.headers().get_one("Authentication") {
                Some(bearer) => bearer.to_string(),
                None => {
                    return Outcome::Failure((
                        Status::Unauthorized,
                        Self::UNAUTHORIZED_ERROR.to_string(),
                    ))
                }
            },
        };

        let (sub, id) = match JWT::strip_bearer(&bearer) {
            Ok(details) => match JWT::decrypt_details(details, &keys) {
                Ok(result) => result,
                Err(_) => {
                    return Outcome::Failure((
                        Status::NotFound,
                        Self::UNAUTHORIZED_ERROR.to_string(),
                    ))
                }
            },
            Err(error) => return Outcome::Failure((error, Self::UNAUTHORIZED_ERROR.to_string())),
        };

        let token = match db.get_token(&id).await {
            Ok(jwt) => match jwt.token_from_jwt(&keys) {
                Ok(result) => result,
                Err(_) => return Outcome::Failure((Status::NotFound, Self::NOT_FOUND.to_string())),
            },
            Err(error) => return Outcome::Failure((error, Self::NOT_FOUND.to_string())),
        };

        match token.exp > timestamp_millis() {
            true => {
                let token_id = match token.role {
                    Type::ACCOUNT => {
                        let account = match db.get_account_from_id(&sub).await {
                            Ok(result) => result,
                            Err(error) => {
                                return Outcome::Failure((error, Self::NOT_FOUND.to_string()))
                            }
                        };
                        let id = option!(val -> account.token; {val} | {return Outcome::Failure((Status::NotFound, Self::NOT_FOUND.to_string()))});
                        id
                    }
                    Type::ADMIN => {
                        let admin = match db.get_admin_from_id(&sub).await {
                            Ok(result) => result,
                            Err(error) => {
                                return Outcome::Failure((error, Self::NOT_FOUND.to_string()))
                            }
                        };
                        let id = option!(val -> admin.token; {val} | {return Outcome::Failure((Status::NotFound, Self::NOT_FOUND.to_string()))});
                        id
                    }
                    Type::ATM => {
                        let atm = match db.get_atm_from_id(&sub).await {
                            Ok(result) => result,
                            Err(error) => {
                                return Outcome::Failure((error, Self::NOT_FOUND.to_string()))
                            }
                        };
                        let id = option!(val -> atm.token; {val} | {return Outcome::Failure((Status::NotFound, Self::NOT_FOUND.to_string()))});
                        id
                    }
                };
                if token_id.to_string() == id {
                    return Outcome::Success(token);
                }
                Outcome::Failure((Status::Unauthorized, Self::UNAUTHORIZED_ERROR.to_string()))
            }
            false => Outcome::Failure((Status::Unauthorized, Self::UNAUTHORIZED_ERROR.to_string())),
        }
    }
}
