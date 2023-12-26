//! To use `anyhow` and `thiserror` crate with Rocket, we need to implement `Responder` trait for
//! around them.

use rocket::http::Status;
use rocket::response::{self, Responder};
use rocket::Request;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),
}

impl<'r> Responder<'r, 'static> for Errors {
    fn respond_to(self, request: &Request<'_>) -> response::Result<'static> {
        match self {
            Errors::SqlxError(_) => Status::InternalServerError.respond_to(request),
            Errors::ValidationError(_) => Status::BadRequest.respond_to(request),
        }
    }
}
