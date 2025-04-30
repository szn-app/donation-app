use api_data::server::{self, connection::PostgresEndpointConfig};
use env_logger;
use log;
use std::env;
use std::error::Error;
use tokio;

/// Parse environment variables and arguments
fn parse_args() -> Result<PostgresEndpointConfig, Box<dyn Error>> {
    // Parse database connection endpoints
    let postgres_endpoint_rw =
        env::var("POSTGRESQL_ENDPOINT_RW").expect("POSTGRESQL_ENDPOINT_RW env not set");
    let postgres_endpoint_ro =
        env::var("POSTGRESQL_ENDPOINT_RO").expect("POSTGRESQL_ENDPOINT_RO env not set");
    let postgres_endpoint_r =
        env::var("POSTGRESQL_ENDPOINT_R").expect("POSTGRESQL_ENDPOINT_R env not set");

    Ok(PostgresEndpointConfig {
        rw: postgres_endpoint_rw,
        ro: postgres_endpoint_ro,
        r: postgres_endpoint_r,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    let log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    // Parse command line arguments
    let config = parse_args()?;

    // Run the server with parsed arguments
    server::run_server(config).await
}
