use std::sync::Once;

use sqlx::{PgConnection, Connection};
use zero2prod::configuration::get_configuration;

const PORT: u16 = 8080;

static INIT: Once = Once::new();

async fn spawn_app() {
    INIT.call_once(|| {
        let server = zero2prod::startup::run(PORT).expect("Failed to bind address.");

        // tokio will spawn this as a background task
        let _ = tokio::spawn(server);
    });
}

fn base_url() -> String {
    format!("http://127.0.0.1:{PORT}")
}

#[tokio::test]
async fn health_check_works() {
    spawn_app().await;

    let client = reqwest::Client::new();

    let resp = client
        .get(format!("{}/health_check", base_url()))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

#[tokio::test]
async fn subscibe_returns_a_200_for_valid_form_data() {
    spawn_app().await;
    
    let configuraation = get_configuration().expect("Failed to read configuration");
    let connection_string = configuraation.database.connection_string();
    let _ = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", base_url()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", base_url()))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with a 400 BAD REQUEST when the payload was {}",
            error_message
        )
    }
}

