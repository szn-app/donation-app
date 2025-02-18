use axum::{
    extract::{Form, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

// redundant for debugging/example purpose
#[derive(serde::Deserialize, Debug)]
pub struct Params {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub grant_type: Option<String>,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
}

// params from frontend request
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
    pub code_verifier: String,
}

// response to frontend token exchange
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
}

pub fn routes() -> Router {
    fn ouath2_routes() -> Router {
        Router::new()
            .route("/oauth2_token", get(oauth2_token))
            .route("/oauth2_token", post(oauth2_token))
    }

    Router::new()
        .route("/health/status", get(health_status))
        .merge(ouath2_routes())
        .fallback(handler_404)
}

pub async fn health_status(Query(params): Query<Params>) -> impl IntoResponse {
    dbg!(&params);
    let id = params.client_id.as_deref().unwrap_or("NO ID RECEIVED");
    println!("--> Health status endpoint called with client_id: {}", id);

    Html(format!("Hello, {}", id)).into_response()
}

// Hydra request params
#[derive(Debug)]
struct HydraTokenForm<'a> {
    grant_type: String,
    code: &'a String,
    redirect_uri: &'a String,
    client_id: String,
    client_secret: &'a String,
    code_verifier: &'a String,
}

impl<'a> HydraTokenForm<'a> {
    fn as_tuples(&self) -> [(&str, &str); 6] {
        [
            ("grant_type", &self.grant_type),
            ("code", &self.code),
            ("redirect_uri", &self.redirect_uri),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("code_verifier", &self.code_verifier),
        ]
    }
}

// OIDC Relying Party (RP) logic
// TODO: use proper tracing library for production
pub async fn oauth2_token(
    Extension(secret_value): Extension<String>,
    form: Form<TokenRequest>,
) -> impl IntoResponse {
    println!("--> oauth2_token endpoint called with payload: {:?}", form);

    let hydra_token_url = "http://hydra-public.auth.svc/oauth2/token";
    let hydra_client = reqwest::Client::builder().build().unwrap();

    let hydra_form = HydraTokenForm {
        grant_type: "authorization_code".to_string(),
        code: &form.code,
        redirect_uri: &form.redirect_uri,
        client_id: "frontend-client".to_string(),
        client_secret: &secret_value,
        code_verifier: &form.code_verifier,
    };

    let response = hydra_client
        .post(hydra_token_url)
        .form(&hydra_form.as_tuples())
        .send()
        .await;

    match response {
        Ok(res) => {
            // Debugging
            {
                // println!("[debug] {:?}", &res);
                // let res_body = res.text().await.unwrap_or_default();
                // if let Ok(json) = serde_json::from_str::<serde_json::Value>(&res_body) {
                //     println!("Response JSON: {:?}", res_body);
                // }
            }

            match res.json::<TokenResponse>().await {
                Ok(token) => Json(token).into_response(),
                Err(e) => {
                    println!("--> Failed to parse token response: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(e) => {
            println!("--> Request to hydra token endpoint failed: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// TODO: refresh token endpoint

pub async fn handler_404() -> impl IntoResponse {
    println!("--> fallback 404 handler called");

    (
        StatusCode::NOT_FOUND,
        Html("<h1>404</h1><p>Nothing to see here</p>"),
    )
}
