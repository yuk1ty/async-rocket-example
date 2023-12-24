//! To use `anyhow` and `thiserror` crate with Rocket, we need to implement `Responder` trait for
//! around them.

use rocket::http::Status;
use rocket::response::{self, Responder};
use rocket::Request;
use thiserror::Error;

pub type Result<T = ()> = std::result::Result<T, AnyhowError>;

#[derive(Debug)]
pub struct AnyhowError(pub anyhow::Error);

impl<E> From<E> for AnyhowError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        AnyhowError(error.into())
    }
}

impl<'r> Responder<'r, 'static> for AnyhowError {
    fn respond_to(self, request: &Request<'_>) -> response::Result<'static> {
        response::Debug(self.0).respond_to(request)
    }
}

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
