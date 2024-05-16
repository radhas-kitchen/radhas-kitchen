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
use services::auth::AuthService;
use sqlx::postgres::PgPoolOptions;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), StartError> {
    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&dotenv!("DATABASE_URL"))
            .await?,
    );

    let auth = AuthServer::new(AuthService::new(pool));

    info!("Server started at localhost:50051");

    Server::builder()
        .add_service(auth)
        .serve(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            50051,
        ))
        .await
        .map_err(|err| {
            error!("Failed to start server: {}", err);
            StartError::from(err)
        })?;

    Ok(())
}
