//! src/startup.rs

use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

use crate::routes::{health_check, subscriptions};

pub fn run(listener: TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
    })
    .listen(listener)?
    .run();
    Ok(server)
}