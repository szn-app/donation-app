use axum;
use axum::extract::Json;
use axum::response::Json as JsonResponse;
use axum::Router;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostData {
    pub id: u64,
    pub name: String,
}

pub async fn post_data(data: Json<PostData>) -> JsonResponse<PostData> {
    JsonResponse(data.0)
}

pub async fn get_data() -> JsonResponse<PostData> {
    JsonResponse(PostData {
        id: 1,
        name: "Hello, World!".to_string(),
    })
}

pub fn routes() -> Router {
    Router::new()
        .route("/post_data", axum::routing::post(post_data))
        .route("/get_data", axum::routing::get(get_data))
}
