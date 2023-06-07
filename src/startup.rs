use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgConnection;

use crate::routes;

pub fn run(port: u16, connection: PgConnection) -> Result<Server, std::io::Error> {
    // This will bind the connection to a smart pointer
    // Look up ARC (Atomic Reference Counted pointers to learn more
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/subscriptions", web::post().to(routes::subscribe))
            .route("/health_check", web::get().to(routes::health_check))
            .app_data(connection.clone())
    })
    .bind(("127.0.0.1", port))?
    .run();

    Ok(server)
}
