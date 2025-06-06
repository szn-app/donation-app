use super::connection::{KetoChannelGroup, PostgresPool};
use crate::api::graphql;
use crate::api::rest;
use axum;
use http;
use serde_json;
use tokio;
use tower_http;

// Debug middleware function that modifies headers
async fn debug_header_middleware(
    mut request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    // Only apply this middleware in debug mode
    #[cfg(debug_assertions)]
    {
        let headers = request.headers_mut();

        // overwride user id from header, if provided
        if let Some(source_value) = headers.get("debug-app-user-id") {
            if let Ok(modified_value) = http::HeaderValue::from_str(&format!(
                "{}",
                source_value.to_str().expect("header value issue")
            )) {
                headers.insert(http::HeaderName::from_static("app-user-id"), modified_value);
            }
        }
    }

    next.run(request).await
}

pub async fn start_http_server(
    postgres_pool_group: PostgresPool,
    keto_channel_group: KetoChannelGroup,
    app_endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    const http_addr: &str = "0.0.0.0:8081";

    // TODO: add tracer for subscribers and observability

    // allow calling api subdomain from root domain
    let cors_layer = tower_http::cors::CorsLayer::new()
        // NOTE: wild card origin headers do not work well in all browsers
        .allow_origin(app_endpoint.parse::<http::header::HeaderValue>().unwrap())
        // NOTE: methods should be explicitely mentioned (tower_http::cors::Any won't work on browsers) when credentials are passed (e.g. Authorization headers) otherwise request would be blocked by browser.
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let fallback_router = axum::Router::new()
        .fallback(handle_not_found)
        .layer(cors_layer);

    let rest_routes =
        rest::routes(app_endpoint).layer(axum::Extension(postgres_pool_group.clone()));

    let graphql_routes = graphql::routes(app_endpoint, postgres_pool_group, keto_channel_group);

    // NOTE: any nested endpoint when accesses through the gateway must be setup in the authorization setting of Ory Oathkeeper access control.
    let http_app = axum::Router::new()
        .nest("/rest", rest_routes)
        .nest("/graphql", graphql_routes)
        .merge(fallback_router)
        .layer(axum::middleware::from_fn(debug_header_middleware));

    let listener = tokio::net::TcpListener::bind(http_addr).await?;

    log::info!("HTTP server running on http://{}", http_addr);
    axum::serve(listener, http_app.into_make_service()).await?;

    Ok(())
}

/// Handles routes that don't match any defined routes
pub async fn handle_not_found(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        axum::Json(serde_json::json!({
            "error": "Not Found",
            "message": format!("The requested path '{}' could not be found", uri),
            "status_code": 404
        })),
    )
}
