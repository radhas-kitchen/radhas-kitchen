use actix_web::http::StatusCode;

use crate::prelude::*;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Database error: {error}")]
    DatabaseError {
        #[from]
        error: sqlx::Error,
        #[location]
        location: &'static Location<'static>,
    },

    #[error("User not found")]
    UserNotFound,

    #[error("Bad password")]
    BadPassword,
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::UserNotFound => StatusCode::NOT_FOUND,
            AuthError::BadPassword => StatusCode::UNAUTHORIZED,
        }
    }
}
