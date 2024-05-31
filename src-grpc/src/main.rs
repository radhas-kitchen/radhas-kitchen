extern crate cuid2;
extern crate dotenvy;
extern crate dotenvy_macro;
extern crate log;
extern crate prost;
extern crate serde;
extern crate serde_json;
extern crate sha256;
extern crate skuld;
extern crate sqlx;
extern crate thiserror;
extern crate tokio;
extern crate tonic;
extern crate tonic_reflection;

mod error;
mod model;
mod prelude;
mod proto;
mod services;

use error::StartError;
use prelude::*;
use services::{auth::AuthService, health::HealthService, jobs::JobsService};
use skuld::log::SkuldLogger;
use sqlx::postgres::PgPoolOptions;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), StartError> {
    SkuldLogger::new(PathBuf::from("log.txt"))
        .unwrap()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&dotenv!("DATABASE_URL"))
            .await?,
    );

    let auth = AuthServer::new(AuthService::new(Arc::clone(&pool)));
    let jobs = JobsServer::new(JobsService::new(Arc::clone(&pool)));
    let health = HealthServer::new(HealthService);

    info!(
        "Starting server at 127.0.0.1:{}",
        std::env::var("PORT").unwrap_or("50051".to_string())
    );

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::DESCRIPTOR)
        .build()?;

    Server::builder()
        .add_service(reflection)
        .add_service(auth)
        .add_service(jobs)
        .add_service(health)
        .serve(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            std::env::var("PORT")
                .ok()
                .map(|s| s.parse())
                .and_then(Result::ok)
                .unwrap_or(50051),
        ))
        .await
        .map_err(|err| {
            error!("Failed to start server: {}", err);
            StartError::from(err)
        })?;

    Ok(())
}
