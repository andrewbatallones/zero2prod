use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("failed to read configuration.");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
    let port = configuration.application_port;

    run(port, connection_pool)?.await
}
