use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;
use std::sync::Arc;

use crate::routes::{health_check::health_check, subscriptions::subscribe};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    let connection = Arc::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
