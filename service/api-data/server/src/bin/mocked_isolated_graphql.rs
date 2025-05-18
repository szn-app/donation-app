use api_data::api::graphql;
use api_data::server::{
    self,
    connection::{KetoChannelGroup, PostgresPool},
    http::handle_not_found,
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

    const http_addr: &str = "0.0.0.0:8081";
    let listener = tokio::net::TcpListener::bind(http_addr).await?;

    let postgres_pool_group = PostgresPool::new_mock();
    let keto_channel_group = KetoChannelGroup::new_mock();

    let graphql_routes = graphql::routes(
        "https://donation-app.local",
        postgres_pool_group,
        keto_channel_group,
    );

    let http_app = axum::Router::new()
        .nest("/graphql", graphql_routes)
        .fallback(handle_not_found);

    log::info!("HTTP server running on http://{}", http_addr);
    axum::serve(listener, http_app.into_make_service()).await?;

    Ok(())
}
