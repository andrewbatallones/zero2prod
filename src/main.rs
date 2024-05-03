use std::net::TcpListener;

use sqlx::PgPool;
use tracing::dispatcher::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // This will make use of the log crate trait to start outputting log information.
    // This will be replaced with the tracing logic
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Sets the level of logging (default is "info")
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // This sets how the logger will be formatted?
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    // This will tell the app what the attach the span to.
    set_global_default(subscriber.into()).expect("Failed to set subscriber");

    let configuration = get_configuration().expect("failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let port = configuration.application_port;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Unable to bind port");

    run(listener, connection_pool)?.await
}
