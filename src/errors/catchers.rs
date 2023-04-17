use crate::models::handlers::Response;
use rocket::{http::Status, Request};

#[catch(500)]
pub fn internal_error() -> Response<String> {
    Response::<String>::new()
        .message("Some error has occurred".to_string())
        .error("Internal Server Error".to_string())
        .status(Status::InternalServerError)
        .clone()
}

#[catch(404)]
pub fn not_found<'r>(req: &'r Request) -> Response<String> {
    Response::<String>::new()
        .message("The requested resource was not found".to_string())
        .error("Not Found".to_string())
        .uri(req.uri().to_string())
        .status(Status::NotFound)
        .clone()
}

#[catch(400)]
pub fn bad_request<'r>(req: &'r Request) -> Response<String> {
    Response::<String>::new()
        .message("Bad request was made".to_string())
        .error("Bad Request".to_string())
        .uri(req.uri().to_string())
        .status(Status::BadRequest)
        .clone()
}

#[catch(401)]
pub fn unauthorized<'r>(req: &'r Request) -> Response<String> {
    Response::<String>::new()
        .message("You are not authorized".to_string())
        .error("Unauthorized".to_string())
        .uri(req.uri().to_string())
        .status(Status::Unauthorized)
        .clone()
}

#[catch(409)]
pub fn conflict<'r>(req: &'r Request) -> Response<String> {
    Response::<String>::new()
        .message("A conflict was observed so the request was not successful".to_string())
        .error("Conflict".to_string())
        .uri(req.uri().to_string())
        .status(Status::Conflict)
        .clone()
}

#[catch(403)]
pub fn forbidden<'r>(req: &'r Request) -> Response<String> {
    Response::<String>::new()
        .message("Forbidden Request".to_string())
        .error("Forbidden".to_string())
        .uri(req.uri().to_string())
        .status(Status::Forbidden)
        .clone()
}

#[catch(406)]
pub fn not_acceptable<'r>(req: &'r Request) -> Response<String> {
    Response::<String>::new()
        .message("This request was not acceptable, maybe due to invalid parameters".to_string())
        .error("Not Accepted".to_string())
        .uri(req.uri().to_string())
        .status(Status::NotAcceptable)
        .clone()
}
