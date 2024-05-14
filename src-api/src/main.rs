extern crate actix_cors;
#[macro_use]
extern crate actix_web;
extern crate cuid2;
extern crate dotenvy;
extern crate dotenvy_macro;
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate sha256;
extern crate sqlx;
extern crate thiserror;
extern crate time;

mod error;
mod model;
mod prelude;
mod routes;

use actix_cors::Cors;
use actix_web::{error::InternalError, App, HttpServer};
use error::StartError;
use prelude::*;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), StartError> {
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(dotenv!("DATABASE_URL"))
        .await?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(
                JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _req| {
                        log::error!("Failed to parse JSON: {:?}", err);

                        InternalError::from_response(
                            err,
                            HttpResponse::Conflict().json(json!({
                                "error": "failed to parse JSON body"
                            })),
                        )
                        .into()
                    }),
            )
            .service(routes::auth::post)
            .service(routes::auth::patch)
            .service(routes::user::post)
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 8084))?
    .run()
    .await?;

    Ok(())
}
