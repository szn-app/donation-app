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

    tokio::select! {
        result = api_data::http::start_http_server() => result,
        result = api_data::grpc::start_grpc_server() => result
    }
}
