pub use crate::model::*;
pub use actix_web::{web::*, HttpResponse, Responder, ResponseError};
pub use dotenvy_macro::dotenv;
pub use serde::{Deserialize, Serialize};
pub use sqlx::{prelude::*, Pool, Postgres};
pub use std::panic::Location;
pub use thiserror::Error;
pub use time::PrimitiveDateTime;

#[allow(unused)]
pub mod serde_iso8601 {
    use serde::{Deserializer, Serializer};
    use time::{OffsetDateTime, PrimitiveDateTime};

    pub fn serialize<S: Serializer>(
        datetime: &PrimitiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        time::serde::iso8601::serialize(
            &OffsetDateTime::new_utc(datetime.date(), datetime.time()),
            serializer,
        )
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<OffsetDateTime, D::Error> {
        let datetime = time::serde::iso8601::deserialize(deserializer)?;
        Ok(datetime.into())
    }
}
