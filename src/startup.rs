use actix_web::{dev::Server, web, App, HttpServer};

use crate::routes;

pub fn run(port: u16) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/subscriptions", web::post().to(routes::subscribe))
            .route("/health_check", web::get().to(routes::health_check))
    })
    .bind(("127.0.0.1", port))?
    .run();

    Ok(server)
}
