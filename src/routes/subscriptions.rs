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
// Instead of manually writing spans, we can just add an attribute to the fn to trace the function
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        // `%` using that symbol is telling tracing to use their Display implementation for logging.
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
        )
    )]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match create_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

    // Once done, the trace span will exit()
    // You can enter/exit multiple times, but close only once.
}

async fn create_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
         INSERT INTO subscriptions (id, email, name, subscribed_at)
         VALUES ($1, $2, $3, $4)
         "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    // The .map_err() can change how the error message is returned (can also add logging)
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
