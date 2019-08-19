use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::Error as DieselError;
use regex::Error as RegexError;
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
pub enum ErrorResponse {
    #[display(fmt = "400 Bad Request: {}", _0)]
    BadRequest(String),
    #[display(fmt = "401 Unauthorised")]
    Unauthorised,
    #[display(fmt = "404 Not Found")]
    NotFound,
    #[display(fmt = "500 Internal Server Error")]
    InternalServerError,
    #[display(fmt = "503 Service Unavailable")]
    ServiceUnavailable,
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        match self {
            ErrorResponse::BadRequest(_) => HttpResponse::BadRequest().json(self),
            ErrorResponse::Unauthorised => HttpResponse::Unauthorized().json(self),
            ErrorResponse::NotFound => HttpResponse::NotFound().json(self),
            ErrorResponse::InternalServerError => HttpResponse::InternalServerError().json(self),
            ErrorResponse::ServiceUnavailable => HttpResponse::ServiceUnavailable().json(self),
        }
    }
}

impl From<DieselError> for ErrorResponse {
    fn from(error: DieselError) -> Self {
        log::error!("Diesel error: {:?}", error);

        ErrorResponse::InternalServerError
    }
}

impl From<RegexError> for ErrorResponse {
    fn from(error: RegexError) -> Self {
        log::error!("Regex error: {:?}", error);

        ErrorResponse::InternalServerError
    }
}
