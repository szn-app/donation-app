use super::connection::PostgresPool;
use crate::graphql_api;
use crate::rest_api;

use axum;
use tokio;

pub async fn start_http_server(
    postgres_pool_group: PostgresPool,
) -> Result<(), Box<dyn std::error::Error>> {
    const http_addr: &str = "0.0.0.0:8081";

    let http_app = axum::Router::new()
        .merge(rest_api::routes())
        .layer(axum::Extension(postgres_pool_group.clone()))
        .merge(graphql_api::routes(postgres_pool_group))
        .fallback(handle_not_found);

    let listener = tokio::net::TcpListener::bind(http_addr).await?;

    log::info!("HTTP server running on http://{}", http_addr);
    axum::serve(listener, http_app.into_make_service()).await?;

    Ok(())
}

// Handler for routes that don't match any defined routes
pub async fn handle_not_found() -> (axum::http::StatusCode, &'static str) {
    (axum::http::StatusCode::NOT_FOUND, "not found")
}
