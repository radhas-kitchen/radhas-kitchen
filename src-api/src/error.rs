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

    #[error("IO error: {error}")]
    IoError {
        #[from]
        error: std::io::Error,
        #[location]
        location: &'static Location<'static>,
    },
}
