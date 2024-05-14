use actix_web::http::StatusCode;

use crate::prelude::*;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Database error: {error}")]
    DatabaseError {
        #[from]
        error: sqlx::Error,
        #[location]
        location: &'static Location<'static>,
    },
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::DatabaseError { error, .. } => {
                if error.as_database_error().is_some() {
                    StatusCode::INTERNAL_SERVER_ERROR
                } else {
                    StatusCode::BAD_REQUEST
                }
            }
        }
    }
}
