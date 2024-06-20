use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod config;
mod controllers;
mod models;
mod schema;
mod services;
mod middleware;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(config::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
