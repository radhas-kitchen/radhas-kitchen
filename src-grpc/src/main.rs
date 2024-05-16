extern crate cuid2;
extern crate dotenvy;
extern crate dotenvy_macro;
extern crate log;
extern crate prost;
extern crate serde;
extern crate serde_json;
extern crate sha256;
extern crate sqlx;
extern crate thiserror;
extern crate tokio;
extern crate tonic;

mod error;
mod model;
mod prelude;
mod proto;
mod services;

use error::StartError;
use prelude::*;
use sqlx::postgres::PgPoolOptions;

fn main() {}
