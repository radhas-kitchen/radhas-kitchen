use crate::prelude::*;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("Database error: {error}")]
    DatabaseError {
        #[from]
        error: sqlx::Error,
        #[location]
        location: &'static Location<'static>,
    },

    #[error("Server error: {error}")]
    ServerError {
        #[from]
        error: tonic::transport::Error,
        #[location]
        location: &'static Location<'static>,
    },

    #[error("Reflection error: {error}")]
    ReflectionError {
        #[from]
        error: tonic_reflection::server::Error,
        #[location]
        location: &'static Location<'static>,
    },
}
