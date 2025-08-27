//! src/startup.rs

use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use actix_web::web::Data;
use crate::routes::{health_check, subscriptions};
//use actix_web::dev::Server;
//use sqlx::PgConnection;
use sqlx::PgPool;//PgPll

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<actix_web::dev::Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
            //register the connection as part of the aplication state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}