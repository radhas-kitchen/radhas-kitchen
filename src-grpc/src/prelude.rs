#![allow(unused_imports)]

pub use crate::model::{JobStatus, UserKind, *};
pub use crate::proto::{JobStatus as JobStatusResponse, UserKind as UserKindResponse, *};
pub use dotenvy_macro::dotenv;
pub use log::{debug, error, info, trace, warn};
pub use serde::{Deserialize, Serialize};
pub use sqlx::{prelude::*, Pool, Postgres};
pub use std::panic::Location;
pub use thiserror::Error;
use time::format_description::well_known::Iso8601;
pub use time::{OffsetDateTime, PrimitiveDateTime};
pub use tonic::{Request, Response, Status};

pub trait ToIso8601 {
    fn to_iso8601(&self) -> String;
    fn into_iso8601(self) -> String
    where
        Self: Sized,
    {
        self.to_iso8601()
    }
}

impl ToIso8601 for PrimitiveDateTime {
    fn to_iso8601(&self) -> String {
        self.format(&Iso8601::DEFAULT).unwrap()
    }
}
