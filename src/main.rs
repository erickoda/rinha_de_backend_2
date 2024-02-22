mod routes;
mod schemas;
mod controllers;
mod repository;

use std::env;

use actix_web::{App, HttpServer, web::Data};
use routes::config::services_config;
use sqlx::{PgPool, Pool};
use sqlx::postgres::Postgres;

#[derive(Clone)]
pub struct AppData {
    pool: PgPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(8080);

    let repo = AppData {
        pool: Pool::<Postgres>::connect("postgresql://example:example@localhost:5432/example")
            .await
            .expect("Error on DB connection")
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(repo.clone()))
            .configure(services_config)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}