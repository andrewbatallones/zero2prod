use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

/// POST /subscribe
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_id_log = format!("request_id {} - ", request_id);

    tracing::info!(
        "{}Adding '{}' '{}' as a new subscriber.",
        request_id_log,
        form.email,
        form.name
    );
    // `r#`: Rust String Literals: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
    match sqlx::query!(
        r#"
         INSERT INTO subscriptions (id, email, name, subscribed_at)
         VALUES ($1, $2, $3, $4)
         "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("{}New subscriber details have been saved", request_id_log);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("{}Failed to execute query: {:?}", request_id_log, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
