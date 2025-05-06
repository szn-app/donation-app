use api_data::server::{self, connection::PostgresEndpointConfig};
use env_logger;
use log;
use std::env;
use std::error::Error;
use tokio;

/// Parse environment variables and arguments
fn parse_args() -> Result<PostgresEndpointConfig, Box<dyn Error>> {
    const DEFAULT_ENDPOINT: &str = "localhost:5432";

    // Parse database connection endpoints with default values
    let postgres_endpoint_rw =
        env::var("POSTGRESQL_ENDPOINT_RW").unwrap_or_else(|_| String::from(DEFAULT_ENDPOINT));
    let postgres_endpoint_ro =
        env::var("POSTGRESQL_ENDPOINT_RO").unwrap_or_else(|_| String::from(DEFAULT_ENDPOINT));
    let postgres_endpoint_r =
        env::var("POSTGRESQL_ENDPOINT_R").unwrap_or_else(|_| String::from(DEFAULT_ENDPOINT));

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
