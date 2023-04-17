use rocket::{http::Status, serde::json::to_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T: Serialize + Clone> {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<u16>,
}

impl<T: Serialize + Clone> Response<T> {
    pub const SUCCESS_MESSAGE: &str = "Succeeded";
    pub const FAIL_MESSAGE: &str = "Failed";

    pub fn new() -> Self {
        Response {
            message: None,
            data: None,
            uri: None,
            token: None,
            error: None,
            status: None,
        }
    }

    pub fn success(&mut self) -> &mut Self {
        self.message = Some(Self::SUCCESS_MESSAGE.to_string());
        self
    }

    pub fn fail(&mut self) -> &mut Self {
        self.message = Some(Self::FAIL_MESSAGE.to_string());
        self
    }

    pub fn message(&mut self, message: String) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn data(&mut self, data: T) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.uri = Some(uri);
        self
    }

    pub fn token(&mut self, token: String) -> &mut Self {
        self.token = Some(token);
        self
    }

    pub fn error(&mut self, error: String) -> &mut Self {
        self.error = Some(error);
        self
    }

    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = Some(status.code);
        self
    }
}

impl<T: Serialize + Clone> Clone for Response<T> {
    fn clone(&self) -> Self {
        Response {
            message: self.message.to_owned(),
            data: self.data.clone(),
            uri: self.uri.to_owned(),
            token: self.token.to_owned(),
            error: self.error.to_owned(),
            status: self.status,
        }
    }
}

#[rocket::async_trait]
impl<'r, T: Serialize + Clone> rocket::response::Responder<'r, 'static> for Response<T> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let json = to_string(&self).unwrap_or(format!("\"status\": {}", self.status.unwrap()));
        Ok(
            rocket::response::Response::build_from(json.respond_to(request)?)
                .header(rocket::http::ContentType::JSON)
                .status(Status::from_code(self.status.unwrap()).unwrap_or(Status::Ok))
                .finalize(),
        )
    }
}
