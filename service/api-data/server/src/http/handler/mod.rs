use crate::server::connection::PostgresPool;
use axum::extract::{Extension, Json};
use axum::response::Json as JsonResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostData {
    pub id: u64,
    pub name: String,
}

pub async fn post_data(
    Extension(postgres_pool_group): Extension<PostgresPool>,
    data: Json<PostData>,
) -> JsonResponse<PostData> {
    JsonResponse(data.0)
}

pub async fn get_data(
    Extension(postgres_pool_group): Extension<PostgresPool>,
) -> JsonResponse<PostData> {
    let db_url = std::env::var("POSTGRESQL_ENDPOINT_RW").unwrap_or_else(|_| "Not set".to_owned());
    dbg!(&db_url);
    let db_url = std::env::var("POSTGRESQL_ENDPOINT_RO").unwrap_or_else(|_| "Not set".to_owned());
    dbg!(&db_url);
    let db_url = std::env::var("POSTGRESQL_ENDPOINT_R").unwrap_or_else(|_| "Not set".to_owned());
    dbg!(&db_url);

    JsonResponse(PostData {
        id: 1,
        name: "Hello, World!".to_string(),
    })
}

// Handler for routes that don't match any defined routes
pub async fn handle_not_found() -> (axum::http::StatusCode, &'static str) {
    (axum::http::StatusCode::NOT_FOUND, "not found")
}
