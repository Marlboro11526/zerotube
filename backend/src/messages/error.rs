use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::Error as DieselError;
use log::Level;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Serialize)]
pub enum ErrorResponse {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorised")]
    Unauthorised,
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        match self {
            ErrorResponse::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ErrorResponse::BadRequest(message) => HttpResponse::BadRequest().json(message),
            ErrorResponse::Unauthorised => HttpResponse::Unauthorized().json("Unauthorised"),
        }
    }
}
