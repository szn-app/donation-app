use axum;
use axum::extract::Json;
use axum::response::Json as JsonResponse;
use axum::Router;
use serde::{Deserialize, Serialize};
use tokio;

pub mod model {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct PostData {
        pub id: u64,
        pub name: String,
    }

    pub async fn post_data(data: Json<PostData>) -> JsonResponse<PostData> {
        JsonResponse(data.0)
    }

    pub async fn get_data() -> JsonResponse<PostData> {
        let db_url = std::env::var("POSTGRESQL_URL_RW").unwrap_or_else(|_| "Not set".to_owned());
        dbg!(&db_url);
        let db_url = std::env::var("POSTGRESQL_URL_RO").unwrap_or_else(|_| "Not set".to_owned());
        dbg!(&db_url);
        let db_url = std::env::var("POSTGRESQL_URL_R").unwrap_or_else(|_| "Not set".to_owned());
        dbg!(&db_url);

        JsonResponse(PostData {
            id: 1,
            name: "Hello, World!".to_string(),
        })
    }
}

pub fn routes() -> Router {
    Router::new()
        .route("/post_data", axum::routing::post(model::post_data))
        .route("/get_data", axum::routing::get(model::get_data))
}

// Handler for routes that don't match any defined routes
pub async fn handle_not_found() -> (axum::http::StatusCode, &'static str) {
    (axum::http::StatusCode::NOT_FOUND, "not found")
}

pub async fn start_http_server() -> Result<(), Box<dyn std::error::Error>> {
    let http_app = axum::Router::new()
        .merge(routes())
        .fallback(handle_not_found);

    let http_addr = "0.0.0.0:8081";
    let listener = tokio::net::TcpListener::bind(http_addr).await?;
    log::info!("HTTP server running on http://{}", http_addr);

    axum::serve(listener, http_app.into_make_service()).await?;
    Ok(())
}
