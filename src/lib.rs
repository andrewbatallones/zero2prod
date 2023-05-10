use actix_web::{HttpResponse, HttpServer, App, web};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::health_check;

    #[actix_web::test]
    async fn health_check_succeeds() {
        let response = health_check().await;

        assert!(response.status().is_success())
    }
}
