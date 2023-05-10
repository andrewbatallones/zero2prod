#[tokio::test]
async fn health_check_works() {
    spawn_app().await;

    let client = reqwest::Client::new();

    let resp = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

async fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address.");

    // tokio will spawn this as a background task
    let _ = tokio::spawn(server);
}