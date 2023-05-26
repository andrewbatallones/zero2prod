use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run(8080)?.await
}
