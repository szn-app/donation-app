use crate::server::connection::PostgresPool;
use axum;
use axum::Router;
use tokio;

pub mod handler;

pub fn routes() -> Router {
    Router::new()
        .route("/post_data", axum::routing::post(handler::post_data))
        .route("/get_data", axum::routing::get(handler::get_data))
}

pub async fn start_http_server(
    postgres_pool_group: PostgresPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let http_app = axum::Router::new()
        .merge(routes())
        .fallback(handler::handle_not_found)
        .layer(axum::Extension(postgres_pool_group));

    let http_addr = "0.0.0.0:8081";
    let listener = tokio::net::TcpListener::bind(http_addr).await?;
    log::info!("HTTP server running on http://{}", http_addr);

    axum::serve(listener, http_app.into_make_service()).await?;
    Ok(())
}
