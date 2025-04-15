use api_data::server;
use env_logger;
use log;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    let log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let postgres_endpoint_rw =
        env::var("POSTGRESQL_ENDPOINT_RW").expect("POSTGRESQL_ENDPOINT_RW env not set");
    let postgres_endpoint_ro =
        env::var("POSTGRESQL_ENDPOINT_RO").expect("POSTGRESQL_ENDPOINT_RO env not set");
    let postgres_endpoint_r =
        env::var("POSTGRESQL_ENDPOINT_R").expect("POSTGRESQL_ENDPOINT_R env not set");

    server::run_server(
        &postgres_endpoint_rw,
        &postgres_endpoint_ro,
        &postgres_endpoint_r,
    )
    .await
}
