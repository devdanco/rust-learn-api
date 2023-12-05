use rust_learn_api::configuration::get_configuration;
use rust_learn_api::startup::run;
use rust_learn_api::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("rust-learn-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let config = get_configuration().expect("Failed to read configuration.");
    let db_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, db_pool).await?.await
}
