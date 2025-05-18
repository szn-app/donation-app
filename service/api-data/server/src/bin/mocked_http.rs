use api_data::api::graphql;
use api_data::server::{
    self,
    connection::{KetoChannelGroup, PostgresPool},
    http::{handle_not_found, start_http_server},
};
use env_logger;
use log;
use std::env;
use std::error::Error;
use tokio;
use tower_http;

/// $`curl -I -k -X OPTIONS http://localhost:8081/graphql`
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    let log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let postgres_pool_group = PostgresPool::new_mock();
    let keto_channel_group = KetoChannelGroup::new_mock();

    start_http_server(
        postgres_pool_group,
        keto_channel_group,
        "http://donation-app.local",
    )
    .await?;

    Ok(())
}
