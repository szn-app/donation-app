use api_data::server::{
    self,
    connection::{KetoEndpointConfig, PostgresEndpointConfig},
};
use env_logger;
use log;
use std::env;
use std::error::Error;
use tokio;

/// Parse environment variables and arguments
fn parse_args() -> Result<(PostgresEndpointConfig, KetoEndpointConfig, String), Box<dyn Error>> {
    let k = {
        const DEFAULT_KETO_ENDPOINT: &str = "localhost:4467";

        let keto_endpoint_read =
            env::var("KETO_ENDPOINT_READ").unwrap_or_else(|_| String::from(DEFAULT_KETO_ENDPOINT));
        let keto_endpoint_write =
            env::var("KETO_ENDPOINT_WRITE").unwrap_or_else(|_| String::from(DEFAULT_KETO_ENDPOINT));

        KetoEndpointConfig {
            read: keto_endpoint_read,
            write: keto_endpoint_write,
        }
    };

    let p = {
        const DEFAULT_POSTGRESQL_ENDPOINT: &str = "localhost:5432";

        // Parse database connection endpoints with default values
        let postgres_endpoint_rw = env::var("POSTGRESQL_ENDPOINT_RW")
            .unwrap_or_else(|_| String::from(DEFAULT_POSTGRESQL_ENDPOINT));
        let postgres_endpoint_ro = env::var("POSTGRESQL_ENDPOINT_RO")
            .unwrap_or_else(|_| String::from(DEFAULT_POSTGRESQL_ENDPOINT));
        let postgres_endpoint_r = env::var("POSTGRESQL_ENDPOINT_R")
            .unwrap_or_else(|_| String::from(DEFAULT_POSTGRESQL_ENDPOINT));

        PostgresEndpointConfig {
            rw: postgres_endpoint_rw,
            ro: postgres_endpoint_ro,
            r: postgres_endpoint_r,
        }
    };

    let app_endpoint =
        env::var("APP_ENDPOINT").unwrap_or_else(|_| String::from("https://donation-app.local"));

    Ok((p, k, app_endpoint))
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
