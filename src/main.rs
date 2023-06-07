use sqlx::{PgConnection, Connection};
use zero2prod::{startup::run, configuration::get_configuration};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("failed to read configuration.");    
    let port = configuration.application_port;
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(port, connection)?.await
}
