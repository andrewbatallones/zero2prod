use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // This will make use of the log crate trait to start outputting log information.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let port = configuration.application_port;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Unable to bind port");

    run(listener, connection_pool)?.await
}
