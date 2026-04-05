use crate::routes::{health_check, subscriptions};
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::{Pool, Postgres};
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    connection_pool: Pool<Postgres>,
) -> Result<Server, std::io::Error> {
    let data_connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check::health_check))
            .route("/subscriptions", web::post().to(subscriptions::subscribe))
            .app_data(data_connection_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
